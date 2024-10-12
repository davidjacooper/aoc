#!/usr/bin/perl
use strict;
use warnings;

my @area = ();
my $nRows = 0;
my $nCols;

my $end_row;
my $end_col;


while (!eof(STDIN))
{
    my $line = <>;
    
    $nCols = 0;
    foreach my $ch (split(//, $line))
    {
        if ($ch eq "\n")
        {
            next;
        }
        elsif ($ch eq "S")
        {
            $area[$nRows][$nCols] = 0;
        }
        elsif ($ch eq "E")
        {
            $end_row = $nRows;
            $end_col = $nCols;
            $area[$nRows][$nCols] = 25;
        }
        else
        {
            $area[$nRows][$nCols] = ord($ch) - ord('a');
        }
        $nCols++;
    }
    $nRows++;
}

for (my $i = 0; $i < $nRows; $i++)
{
    for (my $j = 0; $j < $nCols; $j++)
    {
        print($area[$i][$j] . " ")
    }
    print("\n")
}

my %seen = ();
my @current_row_positions = ($end_row);
my @current_col_positions = ($end_col);
my @next_row_positions;
my @next_col_positions;

sub add_position
{
    my $old_i = shift;
    my $old_j = shift;
    my $new_i = shift;
    my $new_j = shift;
    
    my $hash = $new_i * 1000 + $new_j;
    
    if( !$seen{$hash} &&
        $new_i >= 0 && $new_j >= 0 && $new_i < $nRows && $new_j < $nCols &&
        $area[$new_i][$new_j] - $area[$old_i][$old_j] >= -1)
    {
        $seen{$hash} = 1;
        push @next_row_positions, $new_i;
        push @next_col_positions, $new_j;
    }
}


my $n_steps = 0;
outerloop: while (1)
{
    print("Step " . $n_steps . ": ");
    
    if($#current_row_positions == -1)
    {
        print("Failed to find route.\n");
        last;
    }

    @next_row_positions = ();
    @next_col_positions = ();
    
    for(my $p = 0; $p <= $#current_row_positions; $p++)
    {
        my $i = $current_row_positions[$p];
        my $j = $current_col_positions[$p];
        
        print("(" . $i . "," . $j . ") ");
        
        if($area[$i][$j] == 0)
        {
            print("<- possible start location\n");
            last outerloop;
        }
        
        add_position($i, $j, $i - 1, $j);
        add_position($i, $j, $i + 1, $j);
        add_position($i, $j, $i, $j - 1);
        add_position($i, $j, $i, $j + 1);
        
        # Put current position out of reach, so it can't be visited again.
        $area[$i][$j] = 1000;
    }
    print("\n");
    
    @current_row_positions = @next_row_positions;
    @current_col_positions = @next_col_positions;
    $n_steps++;
}

print("No. steps = " . $n_steps . "\n");
