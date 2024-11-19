#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Stage {
    Thousands = 0,
    Hundreds = 1,
    Decades = 2,
    Units = 3,
}


impl From<u8> for Stage {
    fn from(value: u8) -> Self {
        match value {
            0 => {
                Self::Thousands
            },
            1 => {
                Self::Hundreds
            },
            2 => {
                Self::Decades
            },
            3 => {
                Self::Units
            },
            _ => {
                unreachable!("Stage index out of range")
            }
        }
    }
}

// fn main() {
//     let mut input = String::new();
//     io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read");


//     let roman_number: &str = &input.trim().to_uppercase();
//     println!("Roman number: {roman_number}");

//     match roman_to_arabic(roman_number) {
//         Ok(answer) => {
//             println!("Arabic number: {answer}");
//         },
//         Err(err) => {
//             println!("Error: {err}")
//         }
//     }

// }

pub fn roman_to_arabic(roman_number: &str) -> Result<u32, String> {
    let mut repeats_counter: u8 = 0;
    let mut answer: u32 = 0;
    let mut current_sum: u32 = 0;
    let mut stage = Stage::Thousands;
    let mut stage_completed = false;

    for element in roman_number.chars() {
        match element {
            'I' => {
                match i_logic(stage, answer, current_sum, repeats_counter, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'V' => {
                match vld_logic(stage, Stage::Units, answer, current_sum, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'X' => {
                match cx_logic(stage, Stage::Decades, answer, current_sum, repeats_counter, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'L' => {
                match vld_logic(stage, Stage::Decades, answer, current_sum, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'C' => {
                match cx_logic(stage, Stage::Hundreds, answer, current_sum, repeats_counter, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'D' => {
                match vld_logic(stage, Stage::Hundreds, answer, current_sum, stage_completed) {
                    Ok(data) => {
                        (answer, stage, repeats_counter, current_sum, stage_completed) = data;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            'M' => {
                match m_logic(stage, current_sum, repeats_counter, stage_completed) {
                    Ok(data) => {
                        (repeats_counter, current_sum, stage_completed) = data;
                        // stage = Stage::Thousands;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            _ => {
                return Err(String::from("Invalid input"));
            }
        }
        // println!("{:#?}", stage);
        if repeats_counter > 3 {
            return Err(String::from("Invalid input"));
        }
    }
    answer += current_sum;
    Ok(answer)
}

fn vld_logic(stage: Stage, vld_stage: Stage, mut answer: u32, mut current_sum: u32, mut stage_completed: bool) -> Result<(u32, Stage, u8, u32, bool), String>  {
    let current_stage_index = stage as u8;
    let vld_stage_index = vld_stage as u8;
    if vld_stage_index == 0 {
        unreachable!("VLD logic only for HDU");
    }
    let child_element_value: u32 = 10_u32.pow((3 - vld_stage_index).into());
    let element_value: u32 = 5 * child_element_value;

    if current_stage_index < vld_stage_index {
        answer += current_sum;
        current_sum = element_value;
        stage_completed = false;
    } else if current_stage_index == vld_stage_index {
        if current_sum == child_element_value {
            current_sum = element_value - child_element_value;
            stage_completed = true;
        } else {
            return Err(String::from("Doubled VLD"));
        }
    } else {
        return Err(String::from("Invalid number (VLD)"));
    }

    Ok((answer, Stage::from(vld_stage_index), 0, current_sum, stage_completed))

}

fn cx_logic(mut stage: Stage, cx_stage: Stage, mut answer: u32, mut current_sum: u32, mut repeats_counter: u8, mut stage_completed: bool) -> Result<(u32, Stage, u8, u32, bool), String> {
    let current_stage_index = stage as u8;
    let cx_stage_index = cx_stage as u8;
    if cx_stage_index == 0 || cx_stage_index == 3 {
        unreachable!("CX logic only for HD");
    }
    let child_element_value: u32 = 10_u32.pow((2 - cx_stage_index).into());
    let element_value: u32 = 10 * child_element_value;

    // stage = Stage::from(current_stage_index);

    if current_stage_index < cx_stage_index {
        answer += current_sum;
        stage = Stage::from(cx_stage_index);
        stage_completed = false;
        repeats_counter = 1;
        current_sum = element_value;
    } else if current_stage_index == cx_stage_index {
        if stage_completed {
            return Err(String::from("Stage overflow (HD)"));
        }
        current_sum += element_value;
        repeats_counter += 1;
    } else if current_stage_index == cx_stage_index + 1 {
        if current_sum == child_element_value {
            current_sum = element_value - child_element_value;
            stage_completed = true;
        } else {
            return Err(String::from("Stage error (CX)"));
        }

    } else {
        return Err(String::from("Invalid number (CX)"));
    }
    
    Ok((answer, stage, repeats_counter, current_sum, stage_completed))
}

fn i_logic(stage: Stage, mut answer: u32, mut current_sum: u32, mut repeats_counter: u8, mut stage_completed: bool) -> Result<(u32, Stage, u8, u32, bool), String> {
    let current_stage_index = stage as u8;
    if current_stage_index < 3 {
        answer += current_sum;
        repeats_counter = 1;
        current_sum = 1;
        stage_completed = false;
    } else {
        if stage_completed {
            return Err(String::from("Stage overflow (U)"));
        }
        // if current_sum == 9 || current_sum == 4 {
        //     return Err(String::from("Stage error (I)"));
        // }
        current_sum += 1;
        repeats_counter += 1;
    }

    Ok((answer, Stage::Units, repeats_counter, current_sum, stage_completed))
}

fn m_logic(stage: Stage, mut current_sum: u32, mut repeats_counter: u8, mut stage_completed: bool) -> Result<(u8, u32, bool), String> {
    let current_stage_index = stage as u8;
    match current_stage_index {
        0 => {
            current_sum += 1000;
            repeats_counter += 1;
        },
        1 => {
            if current_sum == 100 {
                current_sum = 900;
                stage_completed = true;
            } else {
                return Err(String::from("Stage error (M)"));
            }
        },
        _ => {
            return Err(String::from("Invalid number (M)"));
        }
    }

    Ok((repeats_counter, current_sum, stage_completed))
}
