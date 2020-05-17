use fantoccini::error::CmdError;
use fantoccini::{Client, Locator};

const HTML: &[u8] = br#"
<!DOCTYPE html>
<html>
<head>
<title>Hidden Click</title>
</head>
<body>
<a style="display: none;">foo</a>
<script>
setTimeout(() => {
let a = document.getElementsByTagName('a')[0];
a.style.display = 'block';
}, 30000);
</script>
</body>
</html>
"#;

#[tokio::main]
async fn main() {
    let mut client = Client::new("http://localhost:4444").await.unwrap();

    let result = run(&mut client).await;

    client.close().await.unwrap();

    result.unwrap();
}

async fn run(client: &mut Client) -> Result<(), CmdError> {
    let encoded = base64::encode(HTML);
    let url = format!("data:text/html;base64,{}", encoded);

    client.goto(&url).await?;

    let elem = client.find(Locator::Css("a")).await?;

    while elem.clone().click().await.is_err() {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
