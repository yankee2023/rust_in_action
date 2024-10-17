fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin,60
    Invalid, data
    ";

    let records = penguin_data.lines();

    for(i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // ヘッダと空白行はスキップ
            continue;
        }

        let fields: Vec<_> = record     // テキスト行で処理開始
            .split(',')                 // レコードをフィールドに分割
            .map(|field| field.trim())  // 各フィールドの空白行を除去
            .collect();                 // フィールドのコレクションを作る
        // if cfg! (debug_assertions) {
        //     eprintln!("debug: {:?} -> {:?}", record, field);
        // }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
