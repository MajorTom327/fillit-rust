use fillit::solve_fillit;

#[test]
pub fn test_resolve_hard() {
  let mut content: Vec<String> = Vec::new();
  for _ in 0..26 {
    content.push(
      String::from("....\n....\n....\n####\n")
    );
  }

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 12);
  assert_eq!(fillit.grid.height, 12);

  let expected: Vec<char> = "\
AAAABBBBCCCC\
DDDDEEEEFFFF\
GGGGHHHHIIII\
JJJJKKKKLLLL\
MMMMNNNNOOOO\
PPPPQQQQRRRR\
SSSSTTTTUUUU\
VVVVWWWWXXXX\
YYYYZZZZ....\
............\
............\
............"
  .split("")
  .map(|c| c.chars().next())
  .filter(|c| *c != None)
  .map(|c| c.unwrap())
  .collect::<Vec<char>>();

  assert_eq!(fillit.grid.cells, expected);

}
