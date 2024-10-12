class Monkey
{
    var items: Array<Int>
    var inspections = 0
    
    let operation: (Int) -> Int
    let testDivisibleBy: Int
    let receiver1: Int
    let receiver2: Int
    
    init(initialItems: Array<Int>,
         operation: @escaping (Int) -> Int,
         testDivisibleBy: Int,
         receiver1: Int,
         receiver2: Int)
    {
        self.items = initialItems
        self.operation = operation
        self.testDivisibleBy = testDivisibleBy
        self.receiver1 = receiver1
        self.receiver2 = receiver2
    }
    
    func takeTurn(monkeyList: Array<Monkey>, modulus: Int)
    {
        for oldWorry in items
        {
            let newWorry = operation(oldWorry) % modulus
            monkeyList[(newWorry % testDivisibleBy == 0) ? receiver1 : receiver2].receive(itemWorry: newWorry)
            inspections += 1
        }
        items.removeAll()
    }
    
    func receive(itemWorry: Int)
    {
        items.append(itemWorry)
    }
    
    func show()
    {
        print("""
            Initial items: \(items)
            testDivisibleBy: \(testDivisibleBy)
            receiver1: \(receiver1)
            receiver2: \(receiver2)
            """
        )
    }
}

class ParseError : Error {}

func getInt(_ lineOpt: String?, _ index: Int) throws -> Int
{
    guard let line = lineOpt else { throw ParseError() }
    return Int(line.split(separator: " ")[index])!
}

func readMonkeyList() throws -> (Array<Monkey>, Int)
{
    var monkeyList = Array<Monkey>()
    var modulus = 1
    
    while let _ = readLine()
    {
        let items = readLine()!.split(separator: ": ")[1].split(separator: ", ").map{ Int($0)! }
        
        var operation: (Int) -> Int
        let opLine = readLine()!.split(separator: " ")
        if let operand = Int(opLine[5])
        {
            operation = (opLine[4] == "*") ? { $0 * operand } : { $0 + operand }
        }
        else
        {
            // Assume we have '* old'
            operation = { $0 * $0 }
        }
        
        let testDivisibleBy = try getInt(readLine(), 3)
        modulus *= testDivisibleBy

        monkeyList.append(Monkey(initialItems: items,
                                 operation: operation,
                                 testDivisibleBy: testDivisibleBy,
                                 receiver1: try getInt(readLine(), 5),
                                 receiver2: try getInt(readLine(), 5)))
       
        readLine() // expected blank line
    }
    
    return (monkeyList, modulus)
}

func doRounds(_ monkeyList: Array<Monkey>, _ modulus: Int)
{
    for round in 1...10000
    {
        print("Round \(round)")
        
        for monkey in monkeyList
        {
            monkey.takeTurn(monkeyList: monkeyList, modulus: modulus)
        }
        
        for (i, monkey) in monkeyList.enumerated()
        {
            print("    monkey \(i): \(monkey.items)")
        }
        print()
    }
}

func showInspections(_ monkeyList: Array<Monkey>)
{
    print("Inspections")
    for (i, monkey) in monkeyList.enumerated()
    {
        print("    monkey \(i): \(monkey.inspections)")
    }
}

func calcMonkeyBusiness(_ monkeyList: Array<Monkey>) -> Int
{
    let sortedMonkeyList = monkeyList.sorted(by: { $0.inspections > $1.inspections }) // Reverse sorted
    return sortedMonkeyList[0].inspections * sortedMonkeyList[1].inspections
}

func main()
{
    do
    {
        let (monkeyList, modulus) = try readMonkeyList()
        monkeyList.forEach { $0.show() }
        
        doRounds(monkeyList, modulus)
        showInspections(monkeyList)
        print("Monkey business = \(calcMonkeyBusiness(monkeyList))")
    }
    catch is ParseError
    {
        print("Parse error")
    }
    catch
    {
        print("Unexpected error")
    }
}

main()
