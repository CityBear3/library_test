import test_lib as test
import time

def collatz(arg: int):
    n = arg
    while n > 1:
        if n % 2 == 0:
            n = n / 2
            print(n)
        else:
            n = 3 * n + 1
            print(n)


    return n


def main():
    a = 9999999999

    s_1 = time.time()
    collatz(a)
    e_1 = time.time()
    print(f"Python:{e_1 - s_1}")

    print('------------------------------------')

    s_2 = time.time()
    test.rs_collatz(a)
    e_2 = time.time()
    print(f"Rust(PyO3):{e_2 - s_2}")


if __name__ == '__main__':
    main()