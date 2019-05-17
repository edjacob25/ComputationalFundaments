static class RandomLetter
{
    static Random _random = new Random();
    public static char GetLetter()
    {
        // This method returns a random lowercase letter.
        // ... Between 'a' and 'z' inclusize.
        int num = _random.Next(0, 26); // Zero to 25
        char let = (char)('a' + num);
        return let;
    }
}

char[] Cipher(char[] plainText, int[] key, int n)
{

    Debug.Assert(plainText.Count() == n);
    Debug.Assert(key.Count() == n);

    var result = new char[n];
    for (var i = 0; i < n; i++)
    {
        result[i] = plainText[key[i]];
    }
    return result;
}

char[] Decipher(char[] cipherText, int[] key, int n)
{

    Debug.Assert(cipherText.Count() == n);
    Debug.Assert(key.Count() == n);

    var result = new char[n];
    for (var i = 0; i < n; i++)
    {
        result[key[i]] = cipherText[i];
    }
    return result;
}

static Random ran = new Random();

int[] GenerateKey(int n = 25)
{
    var result = new int[n];
    result = result.Select( e => -1 ).ToArray();
    for (var i = 0; i < n; i++)
    {
        var num = ran.Next(n);
        if (result.Contains(num))
        {
            i = i - 1;
        }
        else
        {
            Print($"Adding {num} to key");
            result[i] = num;
        }
    }

    return result;
}

char[] CreatePlaintext(int n)
{
    var a = new char[n];
    for (var i = 0; i < n; i++)
    {
        a[i] = (char)('0' + ran.Next(1));
    }
    return a;
}


IList<char[]> CreateDataset(int n)
{
    var list = new List<char[]>();
    for (var i = 0; i < n; i++)
    {
        var txt = CreatePlaintext(25);
        list.Add(txt);
    }    
    return list;
}

IList<char[]> CipherDataset(IList<char[]> dataset, int[] key){
    var result = new List<char[]>();
    foreach (var item in dataset)
    {
        result.Add(Cipher(item, key, 25));
    }
    return result;
}
void SaveDataset(string name, IList<char[]> dataset, IList<char[]> cipherDataset)
{

    using (var writer = new StreamWriter(name))
    {
        for (var i = 0; i < dataset.Count; i++)
        {
            var combine = dataset[i].Concat(cipherDataset[i]).ToList();
            for (var j = 0; j < 50; j++)
            {
                writer.Write(combine[j]);
                if (j < 49)
                {
                    writer.Write(", ");
                }
            }
            if (i < dataset.Count - 1)
            {
                writer.Write('\n');
            }
        }

    }
}

var plaint = new char[] { '0', '1', '0', '1', '0' };
var key = new[] { 3, 4, 2, 0, 1 };

Print(plaint);

var result = Cipher(plaint, key, 5);

Print(result);

var plain = Decipher(result, key, 5);

Print(plain);

//Print(GenerateKey());
var realKey = new int[25] { 11, 22, 18, 6, 8, 5, 4, 0, 24, 9, 3, 16, 7, 19, 10, 12, 20, 23, 21, 17, 13, 15, 2, 1, 14 };

var training = CreateDataset(10_000);

var trainingCipher = CipherDataset(training, realKey);

SaveDataset("data25-Train.csv", training, trainingCipher);

var testing = CreateDataset(10_000);

var testingCipher = CipherDataset(testing, realKey);

SaveDataset("data25-Test.csv", testing, testingCipher);