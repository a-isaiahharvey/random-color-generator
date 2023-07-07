using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Media;
using System;
using Windows.ApplicationModel.DataTransfer;
using Windows.UI;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace RandomColorGenerator
{
    /// <summary>
    /// An empty window that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class MainWindow : Window
    {
        public MainWindow()
        {
            this.InitializeComponent();
            var color = GenerateRandomColor();
            ChangeBackgroundColor(color);
        }

        private void generateButton_Click(object sender, RoutedEventArgs e)
        {
            var color = GenerateRandomColor();
            ChangeBackgroundColor(color);
        }

        private void copyButton_Click(object sender, RoutedEventArgs e)
        {
            DataPackage package = new DataPackage();
            package.SetText(colorNameTextBlock.Text);
            Clipboard.SetContent(package);
        }

        private void ChangeBackgroundColor(Color color)
        {
            mainGrid.Background = new SolidColorBrush(color);
            colorNameTextBlock.Text = color.ToHex();
        }

        private Color GenerateRandomColor()
        {
            Random random = new Random();
            return Color.FromArgb((byte)random.Next(0, 255), (byte)random.Next(0, 255), (byte)random.Next(0, 255), 255);
        }
    }
}

