#include <iostream>
#include "ppointer.h"

void init(double **v) {
  for (int i=0; i<5; i++) {
    for (int j=0; j<5; j++) {
      std::cout << i << "," << j << "=" << v[i][j] << std::endl;
      // v[i][j] = 0.0;
    }
  }
}
