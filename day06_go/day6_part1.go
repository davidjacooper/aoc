package main

import "fmt"
import "bufio"
import "os"
import "math/bits"

func main() {
    line, _ := bufio.NewReader(os.Stdin).ReadString('\n')

    for i := 3; i < len(line); i++ {
        var bitset uint32 = 0
        for j := i - 3; j <= i; j++ {
            bitset |= 1 << (line[j] - 'a')
        }
        if bits.OnesCount32(bitset) == 4 {
            fmt.Printf("%d\n", i + 1)
            break
        }
    }
}
