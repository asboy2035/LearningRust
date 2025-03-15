use std::io;
use std::io::Write;
use rand::Rng;

struct Question {
    text: String,
    answers: Vec<QuestionAnswer>
}

struct QuestionAnswer {
    answer: String,
    is_correct: bool
}

fn main() {
    let questions = vec![
        Question {
            text: String::from("Who is the best person in the world?"),
            answers: vec![
                QuestionAnswer {
                    answer: String::from("Me!"),
                    is_correct: true,
                },
                QuestionAnswer {
                    answer: String::from("No one!"),
                    is_correct: false,
                },
            ],
        },
    ];

    loop {
        let current_question: &Question = load_question(&questions);
        print_question(current_question);

        // Validate
        if let Some(user_answer_index) = get_user_answer(current_question) {
            println!(
                "\nYou selected answer: {}",
                current_question.answers[user_answer_index].answer
            );

            if current_question.answers[user_answer_index].is_correct {
                println!("...which was correct!\n");
            } else {
                println!("aww, looks like that was wrong. Try again next time!");
                return;
            }
        } else {
            println!("uncaught exception")
        }
    }
}

fn load_question(
    questions: &Vec<Question>
) -> &Question {
    let random_index: usize = rand::thread_rng().gen_range(
        0..questions.len()
    );

    &questions[random_index]
}

fn print_question(
    question: &Question
) {
    println!("{}", question.text);

    let mut index: i32 = 1;
    for answer in &question.answers {
        print!("{}", index); // Number
        print!(". "); // Number.
        println!("{}", answer.answer); // Print the answer text

        index += 1;
    }
}

fn get_user_answer(question: &Question) -> Option<usize> {
    loop {
        // Prompt the user for input
        print!("Enter the number of your answer: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is shown immediately

        // Read the input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim the input to remove any surrounding whitespace and parse as an integer
        match input.trim().parse::<usize>() {
            Ok(index) => {
                // Validate that the index is within the valid range
                if index > 0 && index <= question.answers.len() {
                    return Some(index - 1); // Return the 0-based index
                } else {
                    println!("Invalid choice. Please enter a number between 1 and {}.", question.answers.len());
                }
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}