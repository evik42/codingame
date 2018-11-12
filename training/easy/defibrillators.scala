import scala.io.StdIn._

object Defiblirators extends App {
  case class Loc(lon: Double, lat: Double)
  case class Defib(name: String, loc: Loc)
  case class Result(name: String, dist: Double)

  val lon = readLine
  val lat = readLine
  val loc = Loc(lon.replace(',','.').toDouble, lat.replace(',','.').toDouble)
  println((0 until readInt).foldLeft(Result("", Double.MaxValue))(process).name)

  def process(res: Result, _n: Int): Result = {
    val defib = getNameAndLoc(readLine)
    val dist = calculateDistance(loc, defib.loc)
    if (dist < res.dist) Result(defib.name, dist) else res
  }

  def getNameAndLoc(defib:String): Defib = {
    val data = defib.split(";")
    val lon = data(4).replace(',','.').toDouble
    val lat = data(5).replace(',','.').toDouble
    Defib(data(1), Loc(lon, lat))
  }

  def calculateDistance(a: Loc, b: Loc): Double = {
    val x = (b.lon - a.lon) * math.cos ((a.lat + b.lat) / 2)
    val y = b.lat - a.lat
    x * x + y * y
  }
}
