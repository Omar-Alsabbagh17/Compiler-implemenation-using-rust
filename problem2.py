def compare(str1, str2):
    ''''
    This is internal function.
    This funciton accepts two string and returns
    the longest common prefix
    '''

    n1 = len(str1)
    n2 = len(str2)
    min_len = min(n1, n2) # find minimuim length
    prefix = ""
    
    for i in range(0, min_len):
        if (str1[i] != str2[i]):
            break    
        prefix = prefix.__add__(str1[i])

    return (prefix)

#=============================================
def LongestCommonPrefix (arr):
    '''
    This function accepts an array of strings and returns the 
    longest common prefix between those strings.

    Example:
    given array of strings ["apple", "app", "aple", "appl"], the longest common prefix is "ap".

    parameters:
    arr : array of strings

    Return -> string: the longest common prefix between the array of strings
    '''

    n = len(arr)
    common = arr[0]

    for i in range (1, n):
        common = compare(common, arr[i])
        if common == "": # no common prefix
            break

    return common

#=============================================
if __name__ =="__main__":

    arr = ["apple", "appl",
                    "app", "appleorange"]

    # some test cases:
    #  arr = [""]
    #  arr = ["ab", "bb", "c"]

    out = LongestCommonPrefix(arr)

    if out == "": print("There is no common prefix")
    else:    print ("The longest common prefix is: ", out) 
