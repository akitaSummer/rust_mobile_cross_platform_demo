//
//  ContentView.swift
//  Shared
//
//  Created by 刘煜 on 2022/7/3.
//

import SwiftUI
import IOSCodePackage

struct ContentView: View {
    @State var username: String = ""
    @State var result: String = ""

        var body: some View {
            NavigationView {
                Form {
                    Section(header: Text("Test")) {
                        TextField("Username", text: $username)
                    }
                    
                    Section(header: Text("Result")) {
                        HStack {
                            Text("Result")
                            Spacer()
                            Text(result)
                        }
                    }
                    
                    Section {
                        Button(action: {
                            print("Test")
                            result = rust_sm4(username).toString()
                        }) {
                            Text("Test Rust Lib")
                        }
                    }
                }
                .navigationBarTitle("RustLibTest")
            }
        }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
