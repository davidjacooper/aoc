import std.stdio;
import std.algorithm;
import std.container;
import std.format;

void main()
{
    auto stacks = Array!(DList!char)();

    string line;
    while(!((line = readln()).startsWith(" 1")))
    {
	for(int chIdx = 1, stackIdx = 0; chIdx < line.length; chIdx += 4, stackIdx++)
	{
	    if(stackIdx >= stacks.length)
	    {
	    	stacks.insertBack(DList!char());
	    }
	    char crate = line[chIdx];
	    if(crate != ' ')
	    {
	    	stacks[stackIdx].insertBack(crate);
	    }
	}
    }

    showStacks(stacks);

    readln();
    int quantity, src, dest;
    while((line = readln()) !is null)
    {
	formattedRead(line, "move %d from %d to %d\n", quantity, src, dest);
        writefln("Moving %d crates from %d to %d", quantity, src, dest);

	// Convert to zero-based stack indexes
	src--;
	dest--;

	for(; quantity > 0; quantity--)
	{
	    stacks[dest].insertFront(stacks[src].front);
	    stacks[src].removeFront();
	}

	showStacks(stacks);
    }
}

void showStacks(Array!(DList!char) stacks)
{
    int stackN = 1;
    foreach(stack ; stacks)
    {
	writef("%d: ", stackN);
	foreach(char crate ; stack)
	{
	    write(crate);
        }
	writeln();
	stackN++;
    }
}
