use anyhow::Result;
use bytes::{Bytes, BytesMut};
use chrono::Utc;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpListener,
};
use vortex_protocol::{
    HEADER_SIZE, Header,
    tlv::{Tag, download_piece::DownloadPiece, piece_content::PieceContent},
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7100").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (s_reader, s_writer) = socket.into_split();
            let mut reader = BufReader::with_capacity(64 * 1024, s_reader);
            let mut writer = BufWriter::with_capacity(64 * 1024, s_writer);

            let mut header_bytes = BytesMut::with_capacity(HEADER_SIZE);
            header_bytes.resize(HEADER_SIZE, 0);
            reader.read_exact(&mut header_bytes).await.unwrap();
            let header: Header = header_bytes.freeze().try_into().unwrap();
            println!("received header: {:?}", header);

            match header.tag() {
                Tag::DownloadPiece => {
                    // ------------------------------
                    // Recieve DownloadPiece request.
                    // ------------------------------
                    let mut download_piece_bytes =
                        BytesMut::with_capacity(header.length() as usize);
                    download_piece_bytes.resize(header.length() as usize, 0);
                    reader.read_exact(&mut download_piece_bytes).await.unwrap();

                    let download_piece: DownloadPiece =
                        download_piece_bytes.freeze().try_into().unwrap();
                    println!("received DownloadPiece request: {:?}", download_piece,);

                    // ---------------------------
                    // Send PieceContent response.
                    // ---------------------------
                    let content = "Hello World".repeat(1000);
                    let piece_content = PieceContent::new(
                        download_piece.piece_number(),
                        1,
                        content.len() as u64,
                        "crc32:864bbb04".to_string(),
                        "127.0.0.1-foo".to_string(),
                        1,
                        Duration::from_secs(30),
                        Utc::now().naive_utc(),
                    );

                    let piece_content_bytes: Bytes = piece_content.clone().into();
                    let header = Header::new_piece_content(
                        (piece_content_bytes.len() + content.len()) as u32,
                    );
                    let header_bytes: Bytes = header.clone().into();

                    let mut req = BytesMut::with_capacity(HEADER_SIZE + piece_content_bytes.len());
                    req.extend_from_slice(&header_bytes);
                    req.extend_from_slice(&piece_content_bytes);

                    // Write header and piece_content.
                    writer.write_all(&req).await.unwrap();
                    writer.flush().await.unwrap();
                    println!("sent PieceContent: {:?}", piece_content);

                    // Write content.
                    writer.write_all(content.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                    println!("sent content");
                }
                _ => {
                    panic!("unexpected tag: {:?}", header.tag());
                }
            }
        });
    }
}
