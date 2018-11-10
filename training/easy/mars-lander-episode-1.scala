import scala.io.StdIn._

object Player extends App {
  (0 until readInt).foreach(_ => readLine)

  while(true) {
    val Array(x, y, hspeed, vspeed, fuel, rotate, power) = for(i <- readLine split " ") yield i.toInt
    print("0 ")
    println(if (vspeed < -39) 4 else 3)
  }
}

object FuelEfficient extends App {
  val surface = (0 until readInt)
    .map(_ => readLine split " " map {_.toInt})
    .sliding(2)
    .map(ls => if (ls(0)(1) == ls(1)(1)) ls(0)(1) else -1)
    .max

  while (true) {
    val Array(x, y, hspeed, vspeed, fuel, rotate, power) = for(i <- readLine split " ") yield i.toInt
    if (power > 0) {
      print("0 ")
      println(if (vspeed < -39) 4 else 3)
    } else {
      val s = - vspeed + 3.711
      val road = y - surface - (-vspeed)
      val burn = s * 4 + 2.711 * 3 + 1.711 * 2 + 0.711
      val left = road - burn
      val t = math.ceil((s - 34.867) / 0.299)
      val break = s * t + 0.1495 * t * t
      if (t > 0 && s * t + 0.1495 * t * t > left) {
        println("0 4")
      } else {
        println("0 0")
      }
    }
  }
}
