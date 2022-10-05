pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let rows = minefield.len();

    // no rows
    if rows == 0 {
        return result;
    }

    let columns = minefield[0].len();

    // only one rows
    if rows == 1 {
        for (i, &k) in minefield.iter().enumerate() {
            let mut str = String::new();

            for (j, &v) in k.as_bytes().iter().enumerate() {
                if v == 42 {
                    str.push('*');
                    continue;
                }

                let mut count: u32 = 0;

                if j == 0 {
                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                } else if j == columns - 1 {
                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                } else {
                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                }

                if count == 0 {
                    str.push(' ')
                } else {
                    str.push(char::from_digit(count, 10).unwrap());
                }
            }
            result.push(str);
        }

        return result;
    }

    // only one column
    if columns == 1 {
        for (i, &k) in minefield.iter().enumerate() {
            let mut str = String::new();

            for (j, &v) in k.as_bytes().iter().enumerate() {
                if v == 42 {
                    str.push('*');
                    continue;
                }

                let mut count: u32 = 0;

                if i == 0 {
                    if minefield[i + 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                } else if i == rows - 1 {
                    if minefield[i - 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                } else {
                    if minefield[i - 1].as_bytes()[j] == 42 {
                        count += 1;
                    }

                    if minefield[i + 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                }

                if count == 0 {
                    str.push(' ')
                } else {
                    str.push(char::from_digit(count, 10).unwrap());
                }
            }
            result.push(str);
        }

        return result;
    }

    for (i, &k) in minefield.iter().enumerate() {
        let mut str = String::new();
        for (j, &v) in k.as_bytes().iter().enumerate() {
            if v == 42 {
                str.push('*');
                continue;
            }

            let mut count: u32 = 0;

            // top line

            if i == 0 {
                if j == 0 {
                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }

                    // bottom-mid
                    if minefield[i + 1].as_bytes()[j] == 42 {
                        count += 1;
                    }

                    // bottom-right
                    if minefield[i + 1].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                } else if j == columns - 1 {
                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }

                    // bottom-left
                    if minefield[i + 1].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }

                    // bottom-mid
                    if minefield[i + 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                } else {
                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }

                    // bottom-left
                    if minefield[i + 1].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }

                    // bottom-mid
                    if minefield[i + 1].as_bytes()[j] == 42 {
                        count += 1;
                    }

                    // bottom-right
                    if minefield[i + 1].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                }
            }
            // bottom
            else if i == rows - 1 {
                if j == 0 {
                    // top-mid
                    if minefield[i - 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                    // top- right
                    if minefield[i - 1].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }

                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                } else if j == columns - 1 {
                    // top-left
                    if minefield[i - 1].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                    // top-mid
                    if minefield[i - 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                } else {
                    // top-left
                    if minefield[i - 1].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                    // top-mid
                    if minefield[i - 1].as_bytes()[j] == 42 {
                        count += 1;
                    }
                    // top- right
                    if minefield[i - 1].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }

                    // left
                    if minefield[i].as_bytes()[j - 1] == 42 {
                        count += 1;
                    }
                    // right
                    if minefield[i].as_bytes()[j + 1] == 42 {
                        count += 1;
                    }
                }
            } else if j == 0 {
                // top-mid
                if minefield[i - 1].as_bytes()[j] == 42 {
                    count += 1;
                }
                // top- right
                if minefield[i - 1].as_bytes()[j + 1] == 42 {
                    count += 1;
                }

                // right
                if minefield[i].as_bytes()[j + 1] == 42 {
                    count += 1;
                }

                // bottom-mid
                if minefield[i + 1].as_bytes()[j] == 42 {
                    count += 1;
                }

                // bottom-right
                if minefield[i + 1].as_bytes()[j + 1] == 42 {
                    count += 1;
                }
            } else if j == columns - 1 {
                // top-left
                if minefield[i - 1].as_bytes()[j - 1] == 42 {
                    count += 1;
                }
                // top-mid
                if minefield[i - 1].as_bytes()[j] == 42 {
                    count += 1;
                }

                // left
                if minefield[i].as_bytes()[j - 1] == 42 {
                    count += 1;
                }

                // bottom-left
                if minefield[i + 1].as_bytes()[j - 1] == 42 {
                    count += 1;
                }

                // bottom-mid
                if minefield[i + 1].as_bytes()[j] == 42 {
                    count += 1;
                }
            } else {
                // top-left
                if minefield[i - 1].as_bytes()[j - 1] == 42 {
                    count += 1;
                }
                // top-mid
                if minefield[i - 1].as_bytes()[j] == 42 {
                    count += 1;
                }
                // top- right
                if minefield[i - 1].as_bytes()[j + 1] == 42 {
                    count += 1;
                }

                // left
                if minefield[i].as_bytes()[j - 1] == 42 {
                    count += 1;
                }
                // right
                if minefield[i].as_bytes()[j + 1] == 42 {
                    count += 1;
                }

                // bottom-left
                if minefield[i + 1].as_bytes()[j - 1] == 42 {
                    count += 1;
                }

                // bottom-mid
                if minefield[i + 1].as_bytes()[j] == 42 {
                    count += 1;
                }

                // bottom-right
                if minefield[i + 1].as_bytes()[j + 1] == 42 {
                    count += 1;
                }
            }

            if count == 0 {
                str.push(' ')
            } else {
                str.push(char::from_digit(count, 10).unwrap());
            }
        }
        result.push(str);
    }

    return result;
}
