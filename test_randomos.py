import randomos

try:
    os = randomos.get_random_os()
    print(os)
except RuntimeError as e:
    print(f"An error occurred: {e}")
