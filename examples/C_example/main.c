#include <stdio.h>
#include "core_compute_native.h"
#include <stdint.h>

int main(){
  CKernel kernel = {5 , 1 , 1 , "@group(0)@binding(0) var<storage, read_write> v_indices: array<u32>; @compute @workgroup_size(1) fn main(@builtin(global_invocation_id) global_id: vec3<u32>) { v_indices[global_id.x] = v_indices[global_id.x] + 1; }"};
  uint32_t example_data[] = {1 , 2 , 3 , 4 , 5};

  CInfo data;
  data.bind = 0;
  data.group = 0;
  data.data = (uint8_t *) example_data;
  data.data_len = sizeof(uint32_t)*5/sizeof(uint8_t);

  compute0(kernel , data);

  printf("%d\n" , example_data[4]);

  return 0;
}
