// See https://aka.ms/new-console-template for more information
var (totalWrappingPaper, totalRibbon) = GetTotalWrappingPaperAndRibbon();
Console.WriteLine($"Total square feet of wrapping paper: {totalWrappingPaper}, total feet of ribbon: {totalRibbon}");

(long, long) GetTotalWrappingPaperAndRibbon()
{
    long totalWrappingPaper = 0;
    long totalRibbon = 0;
    using (var reader = new StreamReader("../../inputs/day02.txt"))
    {
        while (!reader.EndOfStream)
        {
            var line = reader.ReadLine();
            if (string.IsNullOrEmpty(line))
            {
                continue;
            }

            var rectangularCuboid = (RectangularCuboid)line;

            totalWrappingPaper += rectangularCuboid.SurfaceArea();
            totalWrappingPaper += rectangularCuboid.SmallestSide();

            totalRibbon += rectangularCuboid.WrapLength();
            totalRibbon += rectangularCuboid.BowLength();
        }
    }
    return (totalWrappingPaper, totalRibbon);
}

static class Extensions
{
    public static void Deconstruct(this string dimensions, out int length, out int width, out int height)
    {
        var parts = dimensions.Split('x').Select(int.Parse).ToList();
        length = parts[0];
        width = parts[1];
        height = parts[2];
    }
}

class RectangularCuboid
{
    private readonly int _length;
    private readonly int _width;
    private readonly int _height;

    public RectangularCuboid(int length, int width, int height)
    {
        _length = length;
        _width = width;
        _height = height;
    }

    public static explicit operator RectangularCuboid(string dimensions)
    {
        var (length, width, height) = dimensions;
        return new RectangularCuboid(length, width, height);
    }

    public int SurfaceArea()
    {
        return (2 * _length * _width) +
               (2 * _length * _height) +
               (2 * _width * _height);
    }

    public int SmallestSide()
    {
        var min = Math.Min(_length * _width, _length * _height);
        return Math.Min(min, _width * _height);
    }

    public int WrapLength()
    {
        var min1 = Math.Min(_length, _width);
        var min2 = Math.Min(Math.Max(_length, _width), _height);
        return min1 + min1 + min2 + min2;
    }

    public int BowLength()
    {
        return _length * _width * _height;
    }
}
