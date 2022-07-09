#include <iostream>
#include <string>
#include <vector>

template <typename T>
std::istream& operator>>(std::istream& is, std::vector<T>& v) {
  for (T& x : v) {
    is >> x;
  }
  return is;
}