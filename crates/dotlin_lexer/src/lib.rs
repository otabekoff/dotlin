/// Minimal comment-handling utilities for the Dotlin lexer.
///
/// This provides a small function `remove_comments` which strips
/// end-of-line (`// ...`) and nested block comments (`/* ... */`),
/// preserving other source text and newlines.
pub fn remove_comments(src: &str) -> String {
	let mut out = String::with_capacity(src.len());
	let mut chars = src.chars().peekable();

	while let Some(ch) = chars.next() {
		if ch == '/' {
			match chars.peek() {
				Some('/') => {
					// line comment: consume until newline (include newline)
					// consume the second '/'
					chars.next();
					while let Some(&nc) = chars.peek() {
						let nc = nc;
						chars.next();
						if nc == '\n' {
							out.push('\n');
							break;
						}
					}
					// if EOF reached, just continue (we've dropped the comment)
					continue;
				}
				Some('*') => {
					// block comment (possibly nested)
					// consume the '*'
					chars.next();
					let mut depth: u32 = 1;
					while let Some(nc) = chars.next() {
						if nc == '/' {
							if let Some(&nn) = chars.peek() {
								if nn == '*' {
									// nested start
									chars.next();
									depth += 1;
								}
							}
						} else if nc == '*' {
							if let Some(&nn) = chars.peek() {
								if nn == '/' {
									// end of a block
									chars.next();
									depth = depth.saturating_sub(1);
									if depth == 0 {
										break;
									}
								}
							}
						}
						// otherwise keep consuming until depth reaches 0
					}
					// drop the entire block comment
					continue;
				}
				_ => {
					// not a comment, keep the '/'
					out.push('/');
					continue;
				}
			}
		}

		out.push(ch);
	}

	out
}


#[cfg(test)]
mod tests {
	use super::remove_comments;

	#[test]
	fn strips_line_comments() {
		let src = "let x = 1; // set x to one\nlet y = 2;";
		let got = remove_comments(src);
		assert_eq!(got, "let x = 1; \nlet y = 2;");
	}

	#[test]
	fn strips_block_comment() {
		let src = "let x = /* comment */ 1;";
		let got = remove_comments(src);
		assert_eq!(got, "let x =  1;");
	}

	#[test]
	fn strips_nested_block_comments() {
		let src = "a = /* start /* nested */ end */ b;";
		let got = remove_comments(src);
		assert_eq!(got, "a =  b;");
	}

	#[test]
	fn handles_unterminated_block_comment_by_dropping() {
		let src = "a = /* unterminated...";
		let got = remove_comments(src);
		// unterminated block comment is removed to EOF
		assert_eq!(got, "a = ");
	}
}

