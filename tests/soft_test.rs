use fillit::solve_fillit;

fn get_expected_result(vec: Vec<&str>) -> Vec<char> {
  let inline: String = vec.join("").into();
  inline
    .split("")
    .map(|c| c.chars().next())
    .filter(|c| *c != None)
    .map(|c| c.unwrap())
    .collect::<Vec<char>>()
}

#[test]
pub fn test_resolve_soft_base() {
  let mut content: Vec<String> = Vec::new();
  content.push(
    String::from("....\n....\n..##\n..##\n")
  );

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 2);
  assert_eq!(fillit.grid.height, 2);

  let expected = get_expected_result(vec![
    "AA",
    "AA"
  ]);

  assert_eq!(fillit.grid.cells, expected);
}

#[test]
pub fn test_resolve_soft_0() {
  let mut content: Vec<String> = Vec::new();

  content.push(String::from("..#.\n..#.\n..#.\n..#."));
  content.push(String::from("....\n....\n..##\n..##"));

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 4);
  assert_eq!(fillit.grid.height, 4);

  let expected: Vec<char> = get_expected_result(vec![
    "ABB.",
    "ABB.",
    "A...",
    "A..."
  ]);

  assert_eq!(fillit.grid.cells, expected);
}

#[test]
pub fn test_resolve_soft_1() {
  let mut content: Vec<String> = Vec::new();

  content.push(String::from("....\n.##.\n..#.\n..#."));
  content.push(String::from(".##.\n..#.\n..#.\n...."));

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 4);
  assert_eq!(fillit.grid.height, 4);

  let expected: Vec<char> = get_expected_result(vec![
    "AABB",
    ".A.B",
    ".A.B",
    "...."
  ]);

  assert_eq!(fillit.grid.cells, expected);
}

#[test]
#[ignore]
pub fn test_resolve_soft_2() {
  let mut content: Vec<String> = Vec::new();

  content.push(String::from("....\n.##.\n.##.\n...."));
  content.push(String::from("....\n....\n.##.\n.##."));
  content.push(String::from(".##.\n..#.\n..#.\n...."));

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 4);
  assert_eq!(fillit.grid.height, 4);

  let expected: Vec<char> = get_expected_result(vec![
    "AACC",
    "AA.C",
    "BB.C",
    "BB..",
  ]);

  assert_eq!(fillit.grid.cells, expected);
}

#[test]
pub fn test_resolve_soft_3() {
  let mut content: Vec<String> = Vec::new();

  content.push(String::from("....\n.#..\n##..\n.#.."));
  content.push(String::from("....\n.#..\n##..\n.#.."));
  content.push(String::from("....\n.##.\n.##.\n...."));

  let content = content.join("\n\n");

  let fillit = solve_fillit(content);

  assert_eq!(fillit.grid.width, 5);
  assert_eq!(fillit.grid.height, 5);

  let expected: Vec<char> = get_expected_result(vec![
    ".A.B.",
    "AABB.",
    ".A.B.",
    "CC...",
    "CC..."
  ]);

  assert_eq!(fillit.grid.cells, expected);
}
