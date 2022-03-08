// https://leetcode-cn.com/problems/remove-palindromic-subsequences/

func removePalindromeSub(s string) int {
	if s == "" {
		return 0
	}
	if isPalindrome(s) {
		return 1
	}
	return 2
}

func isPalindrome(s string) bool {
	i,j := 0,len(s)-1
	for i < j {
		if s[i] == s[j] {
			i++
			j--
		}else{
			return false
		}
	}
	return true
}