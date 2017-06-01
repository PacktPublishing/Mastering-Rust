unsigned int count_string(char *str) {
  unsigned int c=0;
  for (c=0; *str != '\0'; c++, *str++);
  return c;
}

