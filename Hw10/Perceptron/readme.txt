The software is provided "As is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

Now that you know the rules, you can test the code by using Matlab.

- Load the training data: training = load('data/data25-Train.csv');
- Select the columns that contain the inputs for training: P = training(:, 1:25);
- Select the columns that contain the expected outputs for training: T = training(:, 26:50);
- Train the perceptron: [W, b] = train(P, T, @hardlim, 5000, 12345);
- Load the testing data: testing = load('data/data25-Test.csv');
- Select the columns that contain the inputs for testing: P = testing(:, 1:25);
- Select the columns that contain the expected outputs for testing: T = testing(:, 26:50);
- Test the perceptron: accuracy(P, T, @hardlim, W, b)

If you need any extra help, please contact me at jcobayliss@tec.mx.