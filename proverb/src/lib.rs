pub fn build_proverb(list: &[&str]) -> String {
  let mut proverb = String::new();
  
  if list.len() == 0
  {
    return proverb
  }

  let mut ending = String::new();

  for element in list.iter()
  {
    if ending != ""
    {
      proverb.push_str(&format!("For want of a {} the {} was lost.\n", ending, element));
    }
    ending = element.to_string();
  }

  proverb.push_str(&format!("And all for the want of a {}.", &list[0]));
  proverb
}
