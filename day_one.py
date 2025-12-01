def find_password():
    password_one = 0  # first puzzle
    password_two = 0  # second puzzle
    position = 50

    with open('input.txt', 'r') as file:
        for line in file:
            direction = -1 if line[0] == 'L' else 1
            number = int(line[1:])
            for i in range(number):
                position = (position + direction) % 100
                if position == 0:
                    password_two += 1

            if position == 0:
                password_one += 1

        print('password_one', password_one)
        print('password_two', password_two)


if __name__ == '__main__':
    find_password()
