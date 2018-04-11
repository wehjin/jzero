#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Question {
    Recall { english: String, progressive: String, kana: String, kanji: Option<String> }
}

pub fn vocabulary_group_a_questions() -> Vec<Question> {
    vec![
        Question::Recall {
            english: "mouth".into(),
            progressive: "kuchi".into(),
            kana: "くち".into(),
            kanji: Some("口".into()),
        },
        Question::Recall {
            english: "eye".into(),
            progressive: "me".into(),
            kana: "め".into(),
            kanji: Some("目".into()),
        },
        Question::Recall {
            english: "ear".into(),
            progressive: "mimi".into(),
            kana: "みみ".into(),
            kanji: Some("耳".into()),
        },
        Question::Recall {
            english: "nose".into(),
            progressive: "hana".into(),
            kana: "はな".into(),
            kanji: Some("鼻".into()),
        },
        Question::Recall {
            english: "face".into(),
            progressive: "kao".into(),
            kana: "かお".into(),
            kanji: Some("顔".into()),
        },
        Question::Recall {
            english: "hand".into(),
            progressive: "te".into(),
            kana: "て".into(),
            kanji: Some("手".into()),
        },
        Question::Recall {
            english: "foot, leg".into(),
            progressive: "ashi".into(),
            kana: "あし".into(),
            kanji: Some("足".into()),
        },
        Question::Recall {
            english: "finger".into(),
            progressive: "yubi".into(),
            kana: "ゆび".into(),
            kanji: Some("指".into()),
        },
        Question::Recall {
            english: "head".into(),
            progressive: "atama".into(),
            kana: "あたま".into(),
            kanji: Some("頭".into()),
        },
        Question::Recall {
            english: "tooth, teeth".into(),
            progressive: "ha".into(),
            kana: "は".into(),
            kanji: Some("歯".into()),
        },
    ]
}

pub fn vocabulary_group_b_questions() -> Vec<Question> {
    vec![
        Question::Recall {
            english: "pillow".into(),
            progressive: "makura".into(),
            kana: "まくら".into(),
            kanji: Some("枕".into()),
        },
        Question::Recall {
            english: "bed".into(),
            progressive: "beddo".into(),
            kana: "ベッド".into(),
            kanji: None,
        },
        Question::Recall {
            english: "futon".into(),
            progressive: "futon".into(),
            kana: "ふとん".into(),
            kanji: Some("布団".into()),
        },
        Question::Recall {
            english: "blanket".into(),
            progressive: "moufu".into(),
            kana: "もうふ".into(),
            kanji: Some("毛布".into()),
        },
        Question::Recall {
            english: "towel".into(),
            progressive: "taoru".into(),
            kana: "タオル".into(),
            kanji: Some("顔".into()),
        },
        Question::Recall {
            english: "bath".into(),
            progressive: "ofuro".into(),
            kana: "おふろ".into(),
            kanji: Some("お風呂".into()),
        },
        Question::Recall {
            english: "soap".into(),
            progressive: "sekken".into(),
            kana: "せっけん".into(),
            kanji: Some("石けん".into()),
        },
        Question::Recall {
            english: "toothbrush".into(),
            progressive: "haburashi".into(),
            kana: "ハブラシ".into(),
            kanji: Some("歯ブラシ".into()),
        },
        Question::Recall {
            english: "mirror".into(),
            progressive: "kagami".into(),
            kana: "かがみ".into(),
            kanji: Some("鏡".into()),
        },
        Question::Recall {
            english: "window".into(),
            progressive: "mado".into(),
            kana: "まど".into(),
            kanji: Some("窓".into()),
        },
    ]
}
