use anyhow::Result;
use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};
use vortex_protocol::{
    HEADER_SIZE, Header, Vortex,
    tlv::Tag,
    tlv::download_piece::DownloadPiece,
    tlv::piece_content::{METADATA_LENGTH_SIZE, PieceContent},
};

#[tokio::main]
async fn main() -> Result<()> {
    let (s_reader, s_writer) = TcpStream::connect("127.0.0.1:7100").await?.into_split();
    let mut reader = BufReader::with_capacity(64 * 1024, s_reader);
    let mut writer = BufWriter::with_capacity(64 * 1024, s_writer);

    // ---------------------------
    // Send DownloadPiece response.
    // ---------------------------
    let req: Bytes = Vortex::DownloadPiece(
        Header::new_download_piece(),
        DownloadPiece::new(
            "150380bc0cf7312d5a9c7c60c550f56ab3404457aef7d031c5d7d08ed93f0af8".to_string(),
            0,
        ),
    )
    .into();
    writer.write_all(&req).await?;
    writer.flush().await?;
    println!("sent DownloadPiece");

    // ------------------------------
    // Recieve PieceContent response.
    // ------------------------------
    let mut header_bytes = BytesMut::with_capacity(HEADER_SIZE);
    header_bytes.resize(HEADER_SIZE, 0);
    reader.read_exact(&mut header_bytes).await.unwrap();
    let header: Header = header_bytes.freeze().try_into().unwrap();

    match header.tag() {
        Tag::PieceContent => {
            let mut metadata_length_bytes = BytesMut::with_capacity(METADATA_LENGTH_SIZE);
            metadata_length_bytes.resize(METADATA_LENGTH_SIZE, 0);
            reader.read_exact(&mut metadata_length_bytes).await.unwrap();

            let metadata_length =
                u32::from_be_bytes(metadata_length_bytes[..].try_into().unwrap()) as usize;

            let mut metadata_bytes = BytesMut::with_capacity(metadata_length);
            metadata_bytes.resize(metadata_length, 0);
            reader.read_exact(&mut metadata_bytes).await.unwrap();

            let mut piece_content_bytes =
                BytesMut::with_capacity(METADATA_LENGTH_SIZE + metadata_length);

            piece_content_bytes.extend_from_slice(&metadata_length_bytes);
            piece_content_bytes.extend_from_slice(&metadata_bytes);
            let piece_content: PieceContent = piece_content_bytes.freeze().try_into().unwrap();
            println!("received PieceContent: {:?}", piece_content);

            // TODO: Read by chunks.
            let mut content = BytesMut::with_capacity(piece_content.metadata().length as usize);
            content.resize(piece_content.metadata().length as usize, 0);
            reader.read_exact(&mut content).await.unwrap();
            println!("received content");
        }
        _ => {
            println!("unexpected tag: {:?}", header.tag());
        }
    }

    Ok(())
}
