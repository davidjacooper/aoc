import 'dart:io';

abstract class OctreeNode
{
    bool contains(int x, int y, int z);
}

class OctreeLeaf extends OctreeNode
{
    int x, y, z;
    
    OctreeLeaf(this.x, this.y, this.z);
    
    @override
    OctreeLeaf add(int x, int y, int z) => this;

    @override
    bool contains(int x, int y, int z) => x == this.x && y == this.y && z == this.z;
    
    String toString() => "($x,$y,$z)";
}


class OctreeComposite extends OctreeNode
{
    final int x1, x2, y1, y2, z1, z2;
    final int xMid, yMid, zMid;
    List<List<List<OctreeNode?>>> children = [[[null, null], [null, null]], [[null, null], [null, null]]];
    
    OctreeComposite(this.x1, this.x2, this.y1, this.y2, this.z1, this.z2) :
        xMid = (x1 + x2) ~/ 2,
        yMid = (y1 + y2) ~/ 2,
        zMid = (z1 + z2) ~/ 2;

    OctreeLeaf add(int x, int y, int z)
    {
        int i = (x <= xMid) ? 0 : 1;
        int j = (y <= yMid) ? 0 : 1;
        int k = (z <= zMid) ? 0 : 1;
        
        var child = children[i][j][k];
        
        if(child == null)
        {
            int x1, x2, y1, y2, z1, z2;
            
            if(x <= xMid)
            {
                x1 = this.x1;
                x2 = xMid;
            }
            else
            {
                x1 = xMid + 1;
                x2 = this.x2;
            }
            
            if(y <= yMid)
            {
                y1 = this.y1;
                y2 = yMid;
            }
            else
            {
                y1 = yMid + 1;
                y2 = this.y2;
            }
            
            if(z <= zMid)
            {
                z1 = this.z1;
                z2 = zMid;
            }
            else
            {
                z1 = zMid + 1;
                z2 = this.z2;
            }
            
            child = (x1 == x2 && y1 == y2 && z1 == z2) ? new OctreeLeaf(x1, y1, z1)
                                                       : new OctreeComposite(x1, x2, y1, y2, z1, z2);
            children[i][j][k] = child;
        }
        
        return (child is OctreeLeaf) ? child
                                     : (child as OctreeComposite).add(x, y, z);
    }
    
    @override
    bool contains(int x, int y, int z)
    {
        return children[(x <= xMid) ? 0 : 1][(y <= yMid) ? 0 : 1][(z <= zMid) ? 0 : 1]?.contains(x,y,z) ?? false;
    }
    
    String toString() =>
        "[ 000${children[0][0][0] ?? "_"} 001${children[0][0][1] ?? "_"} "
        + "010${children[0][1][0] ?? "_"} 011${children[0][1][1] ?? "_"} "
        + "100${children[1][0][0] ?? "_"} 101${children[1][0][1] ?? "_"} "
        + "110${children[1][1][0] ?? "_"} 111${children[1][1][1] ?? "_"} ]";
}



void main()
{
    String? optLine;
    OctreeComposite tree = new OctreeComposite(0, 19, 0, 19, 0, 19);
    List<OctreeLeaf> droplets = [];
    
    while((optLine = stdin.readLineSync()) != null)
    {
        String line = optLine!;
        
        var list = line.split(",");
        var x = int.parse(list[0]),
            y = int.parse(list[1]),
            z = int.parse(list[2]);
            
        droplets.add(tree.add(x, y, z));
    }
    
    int totalSurfaceArea = 0;
    
    for(final droplet in droplets)
    {
        var x = droplet.x;
        var y = droplet.y;
        var z = droplet.z;
        
        int surfaceArea =
            (tree.contains(x - 1, y, z) ? 0 : 1) +
            (tree.contains(x + 1, y, z) ? 0 : 1) +
            (tree.contains(x, y - 1, z) ? 0 : 1) +
            (tree.contains(x, y + 1, z) ? 0 : 1) +
            (tree.contains(x, y, z - 1) ? 0 : 1) +
            (tree.contains(x, y, z + 1) ? 0 : 1);
            
        totalSurfaceArea += surfaceArea;
    }
    
    print("Total surface area = $totalSurfaceArea");
}
