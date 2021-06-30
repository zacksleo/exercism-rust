pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();

    (0..len).fold("".to_string(), |mut cur, index| {
        if index == len - 1 {
            cur.push_str(&format!("And all for the want of a {}.", list[0]));
        } else {
            cur.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[index],
                list[index + 1]
            ));
        }
        cur
    })
}
