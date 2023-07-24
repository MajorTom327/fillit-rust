use fillit::solve_fillit;

#[test]
pub fn test_resolve_soft() {
  let mut content: Vec<String> = Vec::new();
  content.push(
    String::from("....\n....\n..##\n..##\n")
  );

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 2);
  assert_eq!(fillit.grid.height, 2);

  let expected: Vec<char> = "AAAA"
  .split("")
  .map(|c| c.chars().next())
  .filter(|c| *c != None)
  .map(|c| c.unwrap())
  .collect::<Vec<char>>();

  assert_eq!(fillit.grid.cells, expected);

}
