pub enum QuotePolicy {
    QuotePolicyAlways,
    QuotePolicyNone,
    QuotePolicyReserved,
}

const COMMAN_QUOTE_MARK: u8 = b'`';

pub fn always_no_reserve(_str: &String) -> bool {
    false
}

pub fn always_reserve(_str: &String) -> bool {
    true
}

#[derive(Clone)]
pub struct Quoter {
    prefix: u8,
    suffix: u8,
    is_reserved: fn(str: &String) -> bool,
}

impl Quoter {
    pub fn new(prefix: u8, suffix: u8, is_reserved: fn(str: &String) -> bool) -> Self {
        Self {
            prefix,
            suffix,
            is_reserved,
        }
    }

    pub fn common_quoter() -> Self {
        Self {
            prefix: COMMAN_QUOTE_MARK,
            suffix: COMMAN_QUOTE_MARK,
            is_reserved: always_reserve,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.prefix == 0 && self.suffix == 0;
    }

    pub fn quote<'a>(&self, s: &String) -> String {
        // String
        // var buf strings.Builder
        let mut buf = String::new();
        self.quote_to(&mut buf, s);
        return buf;
        // s
    }

    pub fn join(&self, a: Vec<String>, sep: &String) -> String {
        let mut buf = String::new();
        self.join_write(&mut buf, a, sep);
        return buf;
    }

    pub fn join_write(&self, b: &mut String, a: Vec<String>, sep: &String) {
        if a.is_empty() {
            return;
        }

        for (n, str) in a.iter().enumerate() {
            if n > 0 {
                b.push_str(sep.as_str());
            }
            self.quote_to(b, &str.trim().to_string());
        }
    }

    // Trim removes quotes from s
    pub fn trim(&self, s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let trim_start =
            s.trim_start_matches(String::from_utf8(vec![self.prefix]).unwrap().as_str());
        let trim_end =
            trim_start.trim_end_matches(String::from_utf8(vec![self.suffix]).unwrap().as_str());
        trim_end.to_string()
        // // var buf strings.Builder
        // for i := 0; i < len(s); i++ {
        // 	switch {
        // 	case i == 0 && s[i] == q.Prefix:
        // 	case i == len(s)-1 && s[i] == q.Suffix:
        // 	case s[i] == q.Suffix && s[i+1] == '.':
        // 	case s[i] == q.Prefix && s[i-1] == '.':
        // 	default:
        // 		buf.WriteByte(s[i])
        // 	}
        // }
        // return buf.String()
    }

    // QuoteTo quotes the table or column names. i.e. if the quotes are [ and ]
    //   name -> [name]
    //   `name` -> [name]
    //   [name] -> [name]
    //   schema.name -> [schema].[name]
    //   `schema`.`name` -> [schema].[name]
    //   `schema`.name -> [schema].[name]
    //   schema.`name` -> [schema].[name]
    //   [schema].name -> [schema].[name]
    //   schema.[name] -> [schema].[name]
    //   name AS a  ->  [name] AS a
    //   schema.name AS a  ->  [schema].[name] AS a
    pub fn quote_to(&self, buf: &mut String, value: &String) {
        // var i int
        let mut n = 0;
        while n < value.len() {
            let start = Quoter::find_start(value, n);
            if start > n {
                buf.push_str(&value.as_str()[n..start]);
                // if _, err := buf.WriteString(value[i:start]); err != nil {
                // 	return err
                // }
            }
            if start == value.len() {
                return;
            }

            let next_end = Quoter::find_word(value, start);
            self.quote_word_to(buf, &value.as_str()[start..next_end].to_string());
            // if err := q.quoteWordTo(buf, value[start:nextEnd]); err != nil {
            // 	return err
            // }
            // i = nextEnd
            n = next_end;
        }
    }

    fn find_word(v: &String, start: usize) -> usize {
        for (n, v) in v.char_indices() {
            if n >= start && (v == '.' || v == ' ') {
                return n;
            }
        }
        v.len()
        // for j := start; j < len(v); j++ {
        //     switch v[j] {
        //     case '.', ' ':
        //         return j
        //     }
        // }
        // return len(v)
    }

    fn find_start(value: &String, start: usize) -> usize {
        // let mut chars = value.chars();

        if let Some(ch) = value.chars().nth(start) {
            if ch == '.' {
                return start + 1;
            }
            if ch != ' ' {
                return start;
            }

            let mut k: i32 = -1;
            for (n, v) in value.char_indices() {
                if n >= start && v != ' ' {
                    k = n as i32;
                    break;
                }
            }

            if k == -1 {
                return value.len();
            }

            let ch_k = value.chars().nth(k as usize).unwrap();
            let ch_k_1 = value.chars().nth((k + 1) as usize).unwrap();

            if (ch_k == 'A' || ch_k == 'a') && (ch_k_1 == 'S' || ch_k_1 == 's') {
                k += 2;
            }

            for (n, v) in value.char_indices() {
                if n >= (k as usize) && v != ' ' {
                    return n;
                }
            }
        }
        return value.len();
    }

    fn quote_word_to(&self, buf: &mut String, word: &String) {
        // buf.p
        let mut real_word = word.clone();
        let word_bytes = word.as_bytes();
        if (word_bytes[0] == COMMAN_QUOTE_MARK
            && word_bytes[word_bytes.len() - 1] == COMMAN_QUOTE_MARK)
            || (word_bytes[0] == self.prefix && word_bytes[word_bytes.len() - 1] == self.suffix)
        {
            real_word = word.as_str()[1..word.len() - 1].to_string();
        }

        if self.is_empty() {
            buf.push_str(&real_word);
            return;
        }

        let is_reserved = (self.is_reserved)(&real_word);
        if is_reserved && real_word != "*" {
            buf.push(self.prefix.into());
            // if err := buf.WriteByte(q.Prefix); err != nil {
            //     return err
            // }
        }
        buf.push_str(&real_word);
        // if _, err := buf.WriteString(realWord); err != nil {
        //     return err
        // }
        if is_reserved && real_word != "*" {
            buf.push(self.suffix.into());
        }
    }

    // Replace replaces common quote(`) as the quotes on the sql
    pub fn replace(&self, sql: &String) -> String {
        if self.is_empty() {
            return sql.clone();
        }

        // var buf strings.Builder
        // buf.Grow(len(sql))
        let mark: char = COMMAN_QUOTE_MARK.into();

        let mut buf = String::new();

        let mut begin_single_quote: bool = false;
        let chars = &mut sql.chars();
        let len = chars.count();
        let mut n: usize = 0;
        loop {
            if n >= len {
                break;
            }
            let ch = chars.nth(n).unwrap();

            if !begin_single_quote && ch == mark {
                // var j = i + 1
                let mut j = n + 1;
                for (m, ch) in sql.char_indices() {
                    if m >= j && ch == mark {
                        break;
                    }
                    j += 1;
                }
                // for ; j < len(sql); j++ {
                // 	if sql[j] == CommanQuoteMark {
                // 		break
                // 	}
                // }
                let word = sql.as_str()[n + 1..j].to_string();
                let is_reserved = (self.is_reserved)(&word);

                if is_reserved {
                    buf.push(self.prefix.into());
                    // buf.WriteByte(q.Prefix)
                }
                buf.push_str(&word);
                // buf.WriteString(word)
                if is_reserved {
                    // buf.WriteByte(q.Suffix)
                    buf.push(self.suffix.into());
                }
                n = j
            } else {
                if ch == '\'' {
                    begin_single_quote = !begin_single_quote
                }
                buf.push(ch);
                // buf.WriteByte(sql[i])
            }

            n += 1;
        }
        return buf;
    }
}
