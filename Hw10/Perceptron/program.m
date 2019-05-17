
a= [[]]
for _ = 1:1
  sd = randi([1,10000])
  training = load('data/data25-Train-Challenge.csv');
  P = training(:, 1:25);
  T = training(:, 26:50);
  [W, b] = train(P, T, @hardlim, 5000, sd);
  testing = load('data/data25-Test-Challenge.csv');
  P = testing(:, 1:25);
  T = testing(:, 26:50);
  acc = accuracy(P, T, @hardlim, W, b)
  a = [a, [sd; acc]];
end
