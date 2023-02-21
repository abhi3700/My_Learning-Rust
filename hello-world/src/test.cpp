#include <iostream>

using namespace std;

// void print_arr_size(int arr[]) {
//     cout << sizeof(arr) << endl;
// }

void print_arr_size(int (*arr)[5]) {
    cout << "the size of arr in print function: " << sizeof(arr) << "\n";
}

int main() {
    int arr[5] = {1, 2, 3, 4, 5};
    cout << "the size of arr in main function: " << sizeof(arr) << "\n";
    print_arr_size(&arr);
    cout << "the size of arr in main function: " << sizeof(arr) << "\n";
    cout << "the memory address: " << &arr << "\n";
    cout << "the size of (pointer to array) in bytes: " << sizeof(*arr) << "\n";
    cout << "the size of (reference to array) in bytes: " << sizeof(&arr) << "\n";
    cout << "the memory address: " << &(*arr) << "\n";
    return 0;
}
