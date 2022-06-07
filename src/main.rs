extern crate skim;
use skim::prelude::*;
// use std::io::Cursor;

#[derive(Debug, Clone)]
struct Item {
    text: String,
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        if self.text.starts_with("color") {
            ItemPreview::AnsiText(format!("\x1b[31mhello:\x1b[m\n{}", self.text))
        } else {
            ItemPreview::Text(format!("hello:\n{}", self.text))
        }
    }
}
pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("40%"))
        .multi(true)
        // .preview(Some("")) // preview should be specified to enable preview window

        .build()
        .unwrap();

    // let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    tx.send(Arc::new(Item { text: "Onboarding".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Add heroku team member".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Rebuild protos".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Trigger GH action".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Create new flow".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Search plugins".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Search History".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Add a plugin".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Configure staging environmenmt".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Jump to 'macos'".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Jump to 'figterm'".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Create Kafka topic".to_string() })).unwrap();
    tx.send(Arc::new(Item { text: "Start docker".to_string() })).unwrap();

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new())
        .iter()
        .map(|selected_item| (**selected_item).as_any().downcast_ref::<Item>().unwrap().to_owned())
        .collect::<Vec<Item>>();

    for item in selected_items {
        println!("{:?}", item);
    }
}

// Press Control+F to search snippets, shell history & flows using Fig