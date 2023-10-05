fn main() {
    let mut i = 99;
    while i >= 1 {
        if i > 2 {
            print!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.\n\n",i,i,i-1);
        } else {
            if i == 2 {
                print!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottle of beer on the wall.\n\n",i,i,i-1);
            } else {
                print!("{} bottle of beer on the wall,\n{} bottle of beer.\nTake one down, pass it around,\nNo bottles of beer on the wall.",i,i);
            }
        }
        i -= 1;
    }
}
