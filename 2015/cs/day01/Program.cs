// See https://aka.ms/new-console-template for more information
Console.WriteLine($"Current Floor: {part1_GetCurrentFloor()}, Position: {part2_GetPosition()}");

int part1_GetCurrentFloor()
{
    var currentFloor = 0;
    using (var reader = new StreamReader("../../inputs/day01.txt"))
    {
        while (!reader.EndOfStream)
        {
            var c = (char)reader.Read();
            if (c == '(')
            {
                currentFloor++;
            }
            else if (c == ')')
            {
                currentFloor--;
            }
        }
    }
    return currentFloor;
}

int part2_GetPosition()
{
    var position = 1;
    var currentFloor = 0;
    using (var reader = new StreamReader("../../inputs/day01.txt"))
    {
        while (!reader.EndOfStream)
        {
            var c = (char)reader.Read();
            if (c == '(')
            {
                currentFloor++;
            }
            else if (c == ')')
            {
                currentFloor--;
            }

            if (currentFloor == -1)
            {
                break;
            }
            position++;
        }
    }
    return position;
}