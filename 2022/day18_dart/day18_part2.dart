import 'dart:io';


enum CubeType { air, lava, steam }

abstract class OctreeNode
{
    CubeType getType(int x, int y, int z);
}

class OctreeLeaf extends OctreeNode
{
    int x, y, z;
    CubeType type;
    
    OctreeLeaf(this.x, this.y, this.z, this.type);
    
    @override
    OctreeLeaf add(int x, int y, int z) => this;

    @override
    CubeType getType(int x, int y, int z)
    {
        if(x == this.x && y == this.y && z == this.z)
        {
            return this.type;
        }
        else
        {
            return CubeType.air;
        }
    }
    
    String toString() => "($x,$y,$z:$type)";
}


class OctreeComposite extends OctreeNode
{
    final int x1, x2, y1, y2, z1, z2;
    final int xMid, yMid, zMid;
    List<List<List<OctreeNode?>>> children = [[[null, null], [null, null]], [[null, null], [null, null]]];
    
    OctreeComposite(this.x1, this.x2, this.y1, this.y2, this.z1, this.z2) :
        xMid = ((x1 + x2) / 2).floor(),
        yMid = ((y1 + y2) / 2).floor(),
        zMid = ((z1 + z2) / 2).floor();

    OctreeLeaf add(int x, int y, int z, CubeType type)
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
            
            child = (x1 == x2 && y1 == y2 && z1 == z2) ? new OctreeLeaf(x1, y1, z1, type)
                                                       : new OctreeComposite(x1, x2, y1, y2, z1, z2);
            children[i][j][k] = child;
        }
        
        return (child is OctreeLeaf) ? child
                                     : (child as OctreeComposite).add(x, y, z, type);
    }
    
    @override
    CubeType getType(int x, int y, int z)
    {
        return children[(x <= xMid) ? 0 : 1][(y <= yMid) ? 0 : 1][(z <= zMid) ? 0 : 1]?.getType(x,y,z) ?? CubeType.air;
    }
    
    String toString() =>
        "[ 000${children[0][0][0] ?? "_"} 001${children[0][0][1] ?? "_"} "
        + "010${children[0][1][0] ?? "_"} 011${children[0][1][1] ?? "_"} "
        + "100${children[1][0][0] ?? "_"} 101${children[1][0][1] ?? "_"} "
        + "110${children[1][1][0] ?? "_"} 111${children[1][1][1] ?? "_"} ]";
}

class Point
{
    int x, y, z;
    Point(this.x, this.y, this.z);
    
    @override
    bool operator ==(Object other) =>
        other is Point && x == other.x && y == other.y && z == other.z;
    
    @override
    int get hashCode => Object.hash(x, y, z);
    
    String toString() => "($x,$y,$z)";
}

void steamFill(Point bound1, Point bound2, OctreeComposite tree)
{
    var currentPoints = <Point>{new Point(tree.x1, tree.y1, tree.z1)};
    
    while(currentPoints.length > 0)
    {
        var nextPoints = <Point>{};
        var donePoints = <Point>{};
        
        for(final curPt in currentPoints)
        {
            tree.add(curPt.x, curPt.y, curPt.z, CubeType.steam);

            var pointList = <Point>[];
            if(curPt.x >= bound1.x)
            {
                pointList.add(new Point(curPt.x - 1, curPt.y, curPt.z));
            }
            if(curPt.x <= bound2.x)
            {
                pointList.add(new Point(curPt.x + 1, curPt.y, curPt.z));
            }
            if(curPt.y >= bound1.y)
            {
                pointList.add(new Point(curPt.x, curPt.y - 1, curPt.z));
            }
            if(curPt.y <= bound2.y)
            {
                pointList.add(new Point(curPt.x, curPt.y + 1, curPt.z));
            }
            if(curPt.z >= bound1.z)
            {
                pointList.add(new Point(curPt.x, curPt.y, curPt.z - 1));
            }
            if(curPt.z <= bound2.z)
            {
                pointList.add(new Point(curPt.x, curPt.y, curPt.z + 1));
            }
            
            for(final maybeNextPt in pointList)
            {
                if(tree.getType(maybeNextPt.x, maybeNextPt.y, maybeNextPt.z) == CubeType.air)
                {
                    nextPoints.add(maybeNextPt);
                }
            }
            
            currentPoints = nextPoints;
        }
    }
}



void main()
{
    String? optLine;
    OctreeComposite tree = new OctreeComposite(-2, 21, -2, 21, -2, 21);
        // Leave a gap around the edge of the tree, so that the BFS algorithm can find its way
        // around.
        
    List<OctreeLeaf> droplets = [];
    
    // Read file and populate initial 'lava droplets'
    while((optLine = stdin.readLineSync()) != null)
    {
        String line = optLine!;
        
        var list = line.split(",");
        var x = int.parse(list[0]),
            y = int.parse(list[1]),
            z = int.parse(list[2]);
            
        droplets.add(tree.add(x, y, z, CubeType.lava));
    }
    
    // Do BFS to find where the steam would get.
    steamFill(new Point(-1, -1, -1), new Point(20, 20, 20), tree);
    
    int totalSurfaceArea = 0;
    for(final droplet in droplets)
    {
        var x = droplet.x;
        var y = droplet.y;
        var z = droplet.z;
        
        int surfaceArea =
            ((tree.getType(x - 1, y, z) == CubeType.steam) ? 1 : 0) +
            ((tree.getType(x + 1, y, z) == CubeType.steam) ? 1 : 0) +
            ((tree.getType(x, y - 1, z) == CubeType.steam) ? 1 : 0) +
            ((tree.getType(x, y + 1, z) == CubeType.steam) ? 1 : 0) +
            ((tree.getType(x, y, z - 1) == CubeType.steam) ? 1 : 0) +
            ((tree.getType(x, y, z + 1) == CubeType.steam) ? 1 : 0);
            
        totalSurfaceArea += surfaceArea;
    }
    
    print("Total external surface area = $totalSurfaceArea");
}
