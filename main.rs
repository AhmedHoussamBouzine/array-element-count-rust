fn get_occurrences_iteratif(tab: &[i32], element: i32) -> usize {
    if tab.len() == 0 {
        return 0;
    }

    let mut counter = 0;
    for i in 1..tab.len() {
        if tab[i] == element {
            counter += 1;
        }
    }
    counter
}

fn get_occurrences_diviser_pour_regner_tab_non_trie(tab: &[i32], element: i32) -> usize {
    if tab.len() == 0 {
        return 0;
    }

    if tab.len() == 1 {
        return if tab[0] == element { 1 } else { 0 };
    }
    let mid = tab.len() / 2;
    let (left, right) = tab.split_at(mid);
    let left_occurrences = get_occurrences_diviser_pour_regner_tab_non_trie(left, element);
    let right_occurrences = get_occurrences_diviser_pour_regner_tab_non_trie(right, element);

    left_occurrences + right_occurrences
}

fn occurrences_diviser_pour_regner_trie(
    tab: &[i32],
    start: usize,
    end: usize,
    element: i32,
) -> usize {
    if start > end {
        return 0;
    }

    let mid = (start + end) / 2;

    if tab[mid] == element {
        let mut counter = 1;
        for i in (0..mid).rev() {
            if tab[i] == element {
                counter += 1;
            } else {
                break;
            }
        }

        for j in (mid + 1)..tab.len() {
            if tab[j] == element {
                counter += 1;
            } else {
                break;
            }
        }

        return counter;
    } else if tab[mid] < element {
        return occurrences_diviser_pour_regner_trie(tab, mid + 1, end, element);
    } else {
        return occurrences_diviser_pour_regner_trie(tab, start, mid - 1, element);
    }
}

fn main() {
    let tab_non_trie = [2, 0, 1, 6, 3, 4, 5, 6];
    let element_non_trie = 6;

    let resultat_iteratif = get_occurrences_iteratif(&tab_non_trie, element_non_trie);
    println!(
        "Les occurrences de {} en utilisant un algorithme itératif est :  {}",
        element_non_trie, resultat_iteratif
    );

    let resultat_diviser_pour_regner_non_trie =
        get_occurrences_diviser_pour_regner_tab_non_trie(&tab_non_trie, element_non_trie);
    println!("Les occurrences de {} en utilisant un algorithme de type « Diviser pour régner » pour un tableau non trié est :  {}", element_non_trie, resultat_diviser_pour_regner_non_trie);

    let tab_trie = [1, 2, 3, 3, 3, 4, 4, 4, 5, 6];
    let element_trie = 3;

    let resultat_diviser_pour_regner_trie =
        occurrences_diviser_pour_regner_trie(&tab_trie, 0, tab_trie.len() - 1, element_trie);
    println!("Les occurrences de {} en utilisant un algorithme de type « Diviser pour régner » pour un tableau non trié est :  {}", element_trie, resultat_diviser_pour_regner_trie);
}
