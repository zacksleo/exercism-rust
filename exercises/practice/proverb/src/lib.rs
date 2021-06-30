pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let res = list
        .windows(2)
        .map(|x| format!("For want of a {} the {} was lost.", x[0], x[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<Vec<_>>();

    res.join("\n")
}
