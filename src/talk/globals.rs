use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    pub static ref GOODBYE_INPUTS: std::collections::HashSet<&'static str> =
        ["bye", "exit", "goodbye", "quit"].iter().cloned().collect();
    pub static ref DEFAULT_RESPONSES: Vec<&'static str> = vec![
        "Please tell me more.",
        "Let's change focus a bit...Tell me about your family.",
        "Can you elaborate on that?",
        "I see.",
        "Very interesting.",
        "I see. And what does that tell you?",
        "How does that make you feel?",
        "How do you feel when you say that?"
    ];
    pub static ref INTRO_RESPONSES: Vec<&'static str> = vec![
        "Hi %s. I'm just a greeter.",
        "Before we begin, %s, let me tell you something about myself."
    ];
    pub static ref FACTS: Vec<&'static str> = vec![
        "I was created by Joseph Weizenbaum.",
        "I was created in the 1960s.",
        "I am a Rogerian psychotherapist.",
        "I am named after Eliza Doolittle from the play Pygmalion.",
        "I was originally written on an IBM 7094.",
        "I can be accessed in most Emacs implementations with the command M-x doctor.",
        "I was created at the MIT Artificial Intelligence Laboratory.",
        "I was one of the first programs capable of attempting the Turing test.",
        "I was designed as a method to show the superficiality of communication between man and machine."
    ];
    pub static ref REFLECTED_WORDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("am", "are");
        m.insert("was", "were");
        m.insert("i", "you");
        m.insert("i'd", "you would");
        m.insert("i've", "you have");
        m.insert("i'll", "you will");
        m.insert("my", "your");
        m.insert("are", "am");
        m.insert("you've", "I have");
        m.insert("you'll", "I will");
        m.insert("your", "my");
        m.insert("yours", "mine");
        m.insert("you", "me");
        m.insert("me", "you");
        m
    };
    pub static ref REGEX_RESPONSES: Vec<(Regex, Vec<&'static str>)> = vec![
        (
            Regex::new(r"i need (.*)").unwrap(),
            vec![
                "Why do you need %s?",
                "Would it really help you to get %s?",
                "Are you sure you need %s?"
            ]
        ),
        (
            Regex::new(r"why don'?t you ([^\?]*)\??").unwrap(),
            vec![
                "Do you really think I don't %s?",
                "Perhaps eventually I will %s.",
                "Do you really want me to %s?"
            ]
        ),
        (
            Regex::new(r"why can'?t I ([^\?]*)\??").unwrap(),
            vec![
                "Do you think you should be able to %s?",
                "If you could %s, what would you do?",
                "I don't know -- why can't you %s?",
                "Have you really tried?"
            ]
        ),
        (
            Regex::new(r"i can'?t (.*)").unwrap(),
            vec![
                "How do you know you can't %s?",
                "Perhaps you could %s if you tried.",
                "What would it take for you to %s?"
            ]
        ),
        (
            Regex::new(r"i am (.*)").unwrap(),
            vec![
                "Did you come to me because you are %s?",
                "How long have you been %s?",
                "How do you feel about being %s?"
            ]
        ),
        (
            Regex::new(r"i'?m (.*)").unwrap(),
            vec![
                "How does being %s make you feel?",
                "Do you enjoy being %s?",
                "Why do you tell me you're %s?",
                "Why do you think you're %s?"
            ]
        ),
        (
            Regex::new(r"are you ([^\?]*)\??").unwrap(),
            vec![
                "Why does it matter whether I am %s?",
                "Would you prefer it if I were not %s?",
                "Perhaps you believe I am %s.",
                "I may be %s -- what do you think?"
            ]
        ),
        (
            Regex::new(r"what (.*)").unwrap(),
            vec![
                "Why do you ask?",
                "How would an answer to that help you?",
                "What do you think?"
            ]
        ),
        (
            Regex::new(r"how (.*)").unwrap(),
            vec![
                "How do you suppose?",
                "Perhaps you can answer your own question.",
                "What is it you're really asking?"
            ]
        ),
        (
            Regex::new(r"because (.*)").unwrap(),
            vec![
                "Is that the real reason?",
                "What other reasons come to mind?",
                "Does that reason apply to anything else?",
                "If %s, what else must be true?"
            ]
        ),
        (
            Regex::new(r"(.*) sorry (.*)").unwrap(),
            vec![
                "There are many times when no apology is needed.",
                "What feelings do you have when you apologize?"
            ]
        ),
        (
            Regex::new(r"^hello(.*)").unwrap(),
            vec![
                "Hello...I'm glad you could drop by today.",
                "Hello there...how are you today?",
                "Hello, how are you feeling today?"
            ]
        ),
        (
            Regex::new(r"^hi(.*)").unwrap(),
            vec![
                "Hello...I'm glad you could drop by today.",
                "Hi there...how are you today?",
                "Hello, how are you feeling today?"
            ]
        ),
        (
            Regex::new(r"^thanks(.*)").unwrap(),
            vec!["You're welcome!", "Anytime!"]
        ),
        (
            Regex::new(r"^thank you(.*)").unwrap(),
            vec!["You're welcome!", "Anytime!"]
        ),
        (
            Regex::new(r"^good morning(.*)").unwrap(),
            vec![
                "Good morning...I'm glad you could drop by today.",
                "Good morning...how are you today?",
                "Good morning, how are you feeling today?"
            ]
        ),
        (
            Regex::new(r"^good afternoon(.*)").unwrap(),
            vec![
                "Good afternoon...I'm glad you could drop by today.",
                "Good afternoon...how are you today?",
                "Good afternoon, how are you feeling today?"
            ]
        ),
        (
            Regex::new(r"I think (.*)").unwrap(),
            vec![
                "Do you doubt %s?",
                "Do you really think so?",
                "But you're not sure %s?"
            ]
        ),
        (
            Regex::new(r"(.*) friend (.*)").unwrap(),
            vec![
                "Tell me more about your friends.",
                "When you think of a friend, what comes to mind?",
                "Why don't you tell me about a childhood friend?"
            ]
        ),
        (
            Regex::new(r"yes").unwrap(),
            vec!["You seem quite sure.", "OK, but can you elaborate a bit?"]
        ),
        (
            Regex::new(r"(.*) computer(.*)").unwrap(),
            vec![
                "Are you really talking about me?",
                "Does it seem strange to talk to a computer?",
                "How do computers make you feel?",
                "Do you feel threatened by computers?"
            ]
        ),
        (
            Regex::new(r"is it (.*)").unwrap(),
            vec![
                "Do you think it is %s?",
                "Perhaps it's %s -- what do you think?",
                "If it were %s, what would you do?",
                "It could well be that %s."
            ]
        ),
        (
            Regex::new(r"it is (.*)").unwrap(),
            vec![
                "You seem very certain.",
                "If I told you that it probably isn't %s, what would you feel?"
            ]
        ),
        (
            Regex::new(r"can you ([^\?]*)\??").unwrap(),
            vec![
                "What makes you think I can't %s?",
                "If I could %s, then what?",
                "Why do you ask if I can %s?"
            ]
        ),
        (
            Regex::new(r"(.*)dream(.*)").unwrap(),
            vec!["Tell me more about your dream."]
        ),
        (
            Regex::new(r"can I ([^\?]*)\??").unwrap(),
            vec![
                "Perhaps you don't want to %s.",
                "Do you want to be able to %s?",
                "If you could %s, would you?"
            ]
        ),
        (
            Regex::new(r"you are (.*)").unwrap(),
            vec![
                "Why do you think I am %s?",
                "Does it please you to think that I'm %s?",
                "Perhaps you would like me to be %s.",
                "Perhaps you're really talking about yourself?"
            ]
        ),
        (
            Regex::new(r"you'?re (.*)").unwrap(),
            vec![
                "Why do you say I am %s?",
                "Why do you think I am %s?",
                "Are we talking about you, or me?"
            ]
        ),
        (
            Regex::new(r"i don'?t (.*)").unwrap(),
            vec![
                "Don't you really %s?",
                "Why don't you %s?",
                "Do you want to %s?"
            ]
        ),
        (
            Regex::new(r"i feel (.*)").unwrap(),
            vec![
                "Good, tell me more about these feelings.",
                "Do you often feel %s?",
                "When do you usually feel %s?",
                "When you feel %s, what do you do?",
                "Feeling %s? Tell me more."
            ]
        ),
        (
            Regex::new(r"i have (.*)").unwrap(),
            vec![
                "Why do you tell me that you've %s?",
                "Have you really %s?",
                "Now that you have %s, what will you do next?"
            ]
        ),
        (
            Regex::new(r"i would (.*)").unwrap(),
            vec![
                "Could you explain why you would %s?",
                "Why would you %s?",
                "Who else knows that you would %s?"
            ]
        ),
        (
            Regex::new(r"is there (.*)").unwrap(),
            vec![
                "Do you think there is %s?",
                "It's likely that there is %s.",
                "Would you like there to be %s?"
            ]
        ),
        (
            Regex::new(r"my (.*)").unwrap(),
            vec![
                "I see, your %s.",
                "Why do you say that your %s?",
                "When your %s, how do you feel?"
            ]
        ),
        (
            Regex::new(r"you (.*)").unwrap(),
            vec![
                "We should be discussing you, not me.",
                "Why do you say that about me?",
                "Why do you care whether I %s?"
            ]
        ),
        (
            Regex::new(r"why (.*)").unwrap(),
            vec![
                "Why don't you tell me the reason why %s?",
                "Why do you think %s?"
            ]
        ),
        (
            Regex::new(r"i want (.*)").unwrap(),
            vec![
                "What would it mean to you if you got %s?",
                "Why do you want %s?",
                "What would you do if you got %s?",
                "If you got %s, then what would you do?"
            ]
        ),
        (
            Regex::new(r"(.*) mother(.*)").unwrap(),
            vec![
                "Tell me more about your mother.",
                "What was your relationship with your mother like?",
                "How do you feel about your mother?",
                "How does this relate to your feelings today?",
                "Good family relations are important."
            ]
        ),
        (
            Regex::new(r"(.*) father(.*)").unwrap(),
            vec![
                "Tell me more about your father.",
                "How did your father make you feel?",
                "How do you feel about your father?",
                "Does your relationship with your father relate to your feelings today?",
                "Do you have trouble showing affection with your family?"
            ]
        ),
        (
            Regex::new(r"(.*) child(.*)").unwrap(),
            vec![
                "Did you have close friends as a child?",
                "What is your favorite childhood memory?",
                "Do you remember any dreams or nightmares from childhood?",
                "Did the other children sometimes tease you?",
                "How do you think your childhood experiences relate to your feelings today?"
            ]
        ),
        (
            Regex::new(r"(.*)\?").unwrap(),
            vec![
                "Why do you ask that?",
                "Please consider whether you can answer your own question.",
                "Perhaps the answer lies within yourself?",
                "Why don't you tell me?"
            ]
        )
    ];
    pub static ref GOODBYE_RESPONSES: Vec<&'static str> = vec![
        "Goodbye. It was nice talking to you.",
        "Thank you for talking with me.",
        "Thank you, that will be $150. Have a good day!",
        "Goodbye. This was really a nice talk.",
        "Goodbye. I'm looking forward to our next session.",
        "This was a good session, wasn't it – but time is over now. Goodbye.",
        "Maybe we could discuss this over more in our next session? Goodbye.",
        "Good-bye.",
    ];
}
