//
//  GameViewController.swift
//  iOS
//
//  Created by schr3da on 26.11.20.
//

import UIKit
import QuartzCore
import SceneKit

class GameViewController: UIViewController, SCNSceneRendererDelegate {

    var data: OpaquePointer! = nil;
    
    override func viewDidLoad() {
        super.viewDidLoad()
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated);
        
        self.setupState();
        self.setupScene();
    }
        
    func setupScene() {
        let scene = SCNScene(named: "art.scnassets/ship.scn")!;
        
        let cameraNode = SCNNode();
        cameraNode.camera = SCNCamera();
        cameraNode.position = SCNVector3(x: 0, y: 0, z: 15);
        scene.rootNode.addChildNode(cameraNode);
        
        let lightNode = SCNNode();
        lightNode.light = SCNLight();
        lightNode.light!.type = .omni;
        lightNode.position = SCNVector3(x: 0, y: 10, z: 10);
        scene.rootNode.addChildNode(lightNode);
        
        let ambientLightNode = SCNNode();
        ambientLightNode.light = SCNLight();
        ambientLightNode.light!.type = .ambient;
        ambientLightNode.light!.color = UIColor.darkGray;
        scene.rootNode.addChildNode(ambientLightNode);
        
        let scnView = self.view as! SCNView;
        scnView.delegate = self;
        scnView.scene = scene;
        scnView.allowsCameraControl = true;
        scnView.showsStatistics = true;
        scnView.backgroundColor = UIColor.black;
        scnView.isPlaying = true;
    }
    
    func setupState() {
        self.data = init_game_state(
            { (item, isVisible) in
               // print("map renderer");
            }, { (item, progress) in
               // print("building renderer");
            }, { (item) in
               // print("unit renderer");
            }, { (item) in
               // print("preview renderer");
            }, { (item) in
               // print("selection renderer");
            }
        );
    }
    
    override var shouldAutorotate: Bool {
        return true
    }
    
    override var prefersStatusBarHidden: Bool {
        return true
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        if UIDevice.current.userInterfaceIdiom == .phone {
            return .allButUpsideDown
        } else {
            return .all
        }
    }
    
    func renderer(_ renderer: SCNSceneRenderer, updateAtTime time: TimeInterval) {
        update_state(self.data);
    }
    
    func renderer(_ renderer: SCNSceneRenderer, willRenderScene scene: SCNScene, atTime time: TimeInterval) {
        render_state(self.data);
    }
}
