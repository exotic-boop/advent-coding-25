def find_largest_joltage():
    answer = 0

    #First puzzle
    #with open('input.txt', 'r') as file:
    #    for line in file:
    #        print(max(int(line[i] + line[j]) for i in range(len(line)) for j in range(i+1, len(line))))
    #       answer += max(int(line[i] + line[j]) for i in range(len(line)) for j in range(i+1, len(line)))
    
    #seconed puzzle
    with open('input.txt', 'r') as file:
        for line in file:
            line = line.strip()
            start = 0
            battery = ''
            battery_size = 12

            for remaining in range(battery_size, 0, -1):
                end = len(line) - remaining + 1
                max_digit = None
                max_index = None

                for digit_pos in range(start, end):
                    digit = line[digit_pos]
                    if (max_digit is None) or (digit > max_digit):
                        max_digit = digit
                        max_index = digit_pos

                battery += (max_digit)
                start = max_index + 1

            answer += int(battery)
            
    print(answer)


if __name__ == '__main__':
    find_largest_joltage()
