//
//  ContentView.swift
//  RandomColorGenerator
//
//  Created by Allister Harvey on 4/7/23.
//

import SwiftUI

struct ContentView: View {
  @State var rgb = randomRgb()
  var body: some View {
    VStack {
      Button("Generate", action: {
        self.rgb = randomRgb()
      })
      Canvas { context, size in
        context.fill(
          Path(CGRect(origin: .zero, size: size)),
          with: .color(Color(red: rgb.red, green: rgb.green, blue: rgb.blue)))
      }
    }
  }
}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView()
  }
}
