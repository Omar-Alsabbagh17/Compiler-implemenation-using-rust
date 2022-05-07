#include <iostream>

using namespace std;

string compare(string str1, string str2)
{
    /*
    This is internal function.
    This funciton accepts two string and returns
    the longest common prefix
    */
    int n1 = str1.length();
    int n2 = str2.length();
    int min_len = min(n1, n2);
    string prefix = "";
    for (int i  = 0; i < min_len; ++i)
    {
        if (str1[i] != str2[i]) break;
        prefix = prefix+str1[i];
    }
    return prefix;
}

string LongestCommonPrefix(string arr[], int n)
{
    /*
    This function accepts an array of strings and returns the
    longest common prefix between those strings.

    Example:
    given array of strings ["apple", "app", "aple", "appl"], the longest common prefix is "ap".

    parameters:
    arr : array of strings
    n   : size of the array

    Return -> string: the longest common prefix between the array of strings
    */

    string common = arr[0];
    for (int i = 1; i < n; i++)
    {
        common = compare(common, arr[i]);
        if (common == "") break; // no common prefix
    }
    return common;
}

int main()
{
    string arr[] = {"apple", "app", "aplp", "appleor"};
    int n = sizeof(arr)/sizeof(arr[0]); //length of the array

    string out = LongestCommonPrefix(arr, n);
    if (out == "") cout<<"There is no common prefix";
    else cout<<"The longest common prefix is: "<<out;
    return 0;
}
