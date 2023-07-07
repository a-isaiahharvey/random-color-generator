using Windows.UI;

namespace RandomColorGenerator
{
    public static class HexColorExtensions
    {
        public static string ToHex(this Color c) => $"#{c.R:X2}{c.G:X2}{c.B:X2}";
    }
}
