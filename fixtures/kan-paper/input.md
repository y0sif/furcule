# Rough Hook: KANs in chess engines and chess vision

This paper presents, to the best of our knowledge, the first comprehensive empirical evaluation of Kolmogorov-Arnold Networks (KANs) across two distinct chess-related domains: engine evaluation and computer vision classification. Through the Rough Hook project, we implement two modular systems: Rusty Brain (a chess engine with switchable evaluation functions) and Hook Lens (a computer vision pipeline for piece classification). Our experimental framework compares KAN and traditional neural architectures across chess piece images and engine evaluation tasks. Results demonstrate domain-dependent effectiveness: computer vision tasks show KAN advantages with 1.81% accuracy improvement (97.86% vs 96.05%) over CNN+MLP despite 14.3% computational overhead, while infrastructure limitations prevent direct KAN implementation in performance-critical chess engines. These findings provide actionable guidance for KAN adoption in practical applications.

Kolmogorov-Arnold Networks, Neural Architecture Comparison, Chess Engine, Computer Vision, NNUE

## Introduction

The emergence of Kolmogorov-Arnold Networks (KANs)  represents a paradigm shift in neural computation, offering learnable activation functions positioned on network edges rather than nodes, potentially providing superior interpretability and approximation capabilities compared to traditional Multi-Layer Perceptrons (MLPs) . Despite theoretical promise, comprehensive empirical evaluations of KANs in real-world applications remain critically limited.

Chess-related applications provide an ideal testbed for architectural evaluation due to well-defined performance metrics, established benchmarks, and varying computational requirements. Chess engines demand high-speed evaluation with minimal latency, while computer vision systems require robust real-time classification . The domain has benefited from significant advances in AI research, from classical game-playing algorithms to modern deep learning approaches .

This research addresses the gap between theoretical KAN research and practical implementation by developing a comprehensive evaluation framework across two complementary domains. We make the following contributions:

- First systematic comparison of KAN vs traditional architectures in chess-related applications, to the best of our knowledge
- Modular evaluation framework enabling controlled architectural comparison
- Empirical evidence of KAN performance characteristics across diverse computational requirements
- Practical deployment guidelines based on measured accuracy-efficiency trade-offs

## Related Work

### Kolmogorov-Arnold Networks

KANs are inspired by the Kolmogorov-Arnold representation theorem , which states that any multivariate continuous function can be represented as compositions of univariate functions. Unlike MLPs with fixed activation functions, KANs employ learnable spline-based activations on edges, potentially offering enhanced function approximation and interpretability .

**Architectural Characteristics:** KAN performance is influenced by hyperparameters, including grid size (controlling spline resolution) and spline order (determining polynomial degree). These learnable activation functions can be visualized and analyzed to understand network decision-making processes, potentially providing superior interpretability compared to traditional approaches.

**Comparative Performance and Parameter Efficiency:** Recent comprehensive evaluation by Yu et al.  provides a systematic comparison between KAN and MLP across multiple domains under controlled parameter and computational constraints. Their findings indicate that while MLPs generally outperform KANs in most tasks, KANs demonstrate specific advantages in symbolic formula representation, primarily attributed to their B-spline activation functions. Importantly, their methodology of parameter-controlled comparison reveals performance characteristics that emerge specifically under resource constraints, suggesting that architectural advantages may be scale-dependent. This aligns with emerging evidence that KAN benefits become more pronounced when architectures are compressed or constrained to smaller parameter counts, indicating superior parameter efficiency at reduced scales where learnable activation functions can better exploit limited representational capacity.

### Chess Engine Evaluation

Modern chess engines have evolved from handcrafted evaluation (HCE) to neural approaches. Efficiently Updatable Neural Networks (NNUE) revolutionized chess evaluation by combining neural accuracy with real-time efficiency requirements through incremental updates, specialized feature engineering, dual perspective evaluation, and quantization techniques . Stockfish's  integration of NNUE  demonstrated neural evaluation effectiveness, with comprehensive neural network approaches for chess evaluation showing significant performance improvements .

**Technical Implementation:** NNUE addresses computational challenges through incremental activation updates during move making/unmaking, chess-specific feature engineering capturing piece relationships, and dual perspective evaluation from both player viewpoints . These innovations enable real-time neural evaluation in competitive chess engines.

### Computer Vision for Chess

Chess piece recognition presents unique challenges, including class imbalance, visual similarity between pieces, and environmental variation. Traditional approaches rely on classical computer vision techniques with handcrafted features such as edge detection, corner detection, and template matching , while modern deep learning approaches combine CNN feature extraction  with MLP classification  for improved accuracy and robustness .

**Performance Requirements:** Real-time chess vision systems must balance classification accuracy with computational efficiency. Interactive applications require sub-second processing for complete board states, while mobile deployment imposes memory constraints on model size. These requirements necessitate careful architectural optimization and trade-off analysis between accuracy and efficiency.

## System Architecture

The Rough Hook project implements two modular systems designed for independent operation while supporting integrated analysis. Each module maintains its own specialized architecture optimized for domain-specific requirements.

### Rusty Brain: Chess Engine

A modular chess engine supporting different evaluation functions, as illustrated in Figure :

- **Board Representation:** Bitboard-based position representation with separate bitboards for each piece type 
- **Search Algorithm:** Alpha-beta minimax with transposition tables and iterative deepening 
- **Evaluation Functions:** Implementation of both HCE and NNUE evaluation approaches

[htbp]

### Hook Lens: Computer Vision Pipeline

Real-time chessboard detection and piece classification system implementing modular CNN+Classifier architectures:

- **Board Detection:** Canny edge detection  and Hough line transformation  for board localization with perspective correction using OpenCV 
- **Square Segmentation:** Precise extraction of 64 individual squares for piece classification
- **Feature Extraction:** VGG-style CNN  with three convolutional blocks, each containing dual 3×3 convolutions, batch normalization, ReLU activations, 2×2 max pooling, and dropout regularization
- **Classification:** Switchable MLP/KAN classifiers with identical preprocessing pipeline for controlled architectural comparison
- **FEN Generation:** Conversion to Forsyth-Edwards Notation  for engine integration

Figure  illustrates the complete Hook Lens computer vision pipeline from board detection to FEN generation.

[htbp]

## Experimental Methodology

### Controlled Comparison Framework

Each module maintains identical datasets, preprocessing pipelines, and evaluation metrics while varying only neural architecture components. This approach ensures fair comparison by isolating architectural effects from other variables.

### Parameter Matching

Experiments include configurations where parameter counts are matched between architectures to distinguish architectural benefits from capacity effects. Additional experiments explore hyperparameter optimization effectiveness across different KAN configurations:

- **Node Parameter Analysis:** Hidden layer sizes (4, 8, 16, 32) with default hyperparameters to assess capacity scaling
- **Hyperparameter Impact:** Grid size variations (5→2) and spline order adjustments (3→2, 3→0) for computational optimization
- **Baseline Comparison:** MLP architectures with varying configurations (64×16, 128×64, 16×4) for comprehensive benchmarking

### Multi-Metric Evaluation

Performance assessment encompasses accuracy metrics, computational efficiency measures, and domain-specific benchmarks, including classification accuracy, inference speed, training stability, and robustness evaluation.

## Implementation and Results

### Rusty Brain Results

The chess engine successfully implemented NNUE evaluation using the Bullet  framework with a dual perspective 768 feature set and 128 hidden neurons, as illustrated in Figure . Training utilized Stockfish binary packages (sfbinpack) , which contain pre-processed position evaluations from high-quality chess games. Training results demonstrate:

- Single sfbinpack training: ~2400 Elo rating
- Four sfbinpack training: ~2600 Elo rating (200 point improvement)

[htbp]

The framework migration from Burn  to Bullet  proved crucial for competitive performance, with Bullet's native dual perspective feature set handling and efficient data processing contributing significantly to these results. NNUE demonstrated dramatic practical advantages over handcrafted evaluation (HCE): while the HCE implementation (based on Stockfish's evaluation function) proved too slow for interactive play, NNUE achieved competitive Elo ratings (~2400-2600) with sufficient speed for real-time gameplay. This performance gap validates neural evaluation's practical effectiveness, achieving strong playing strength while maintaining playable response times.

KAN integration proved technically infeasible due to:

- Lack of pre-optimized KAN libraries for high-performance applications
- Requirement for custom CUDA kernel development due to complex mathematical operations in learnable activation functions
- Specialized optimization infrastructure is absent in the Bullet framework
- Time and expertise constraints for implementing efficient KAN components from scratch

Unlike the Burn  framework, which provides experimental architecture support, Bullet's focus on production optimization lacks built-in KAN capabilities. Consequently, a direct KAN vs NNUE comparison remains unavailable, highlighting infrastructure limitations for novel architectures in performance-critical applications.

### Hook Lens Results

The computer vision module utilized ~10,000 original chess piece images (227×227×3 pixels), resized to 32×32×3 pixels before augmentation, expanded to ~150,000 through comprehensive augmentation, including rotation, noise addition, contrast adjustment, and blur application. The dataset was split using a 60/20/20 ratio for training, validation, and testing, respectively.

**Architecture Specifications:** Both CNN+MLP and CNN+KAN employ identical VGG-style convolutional feature extractors with three convolutional blocks. Each block consists of two 3×3 convolutional layers with batch normalization, ReLU activations, 2×2 max pooling, and dropout regularization. The CNN weights were frozen after initial training to ensure fair classifier comparison. The implementation utilized PyTorch  for neural network training and Rust  for the overall system architecture.

**Training Protocol:** A two-phase training approach was employed: (1) Initial CNN+MLP training to establish optimal feature extraction weights, (2) CNN layer freezing followed by KAN classifier training with identical preprocessing and feature extraction to isolate architectural effects.

Figure  shows the architectural differences between the CNN+MLP and CNN+KAN approaches used in our comparative analysis.

[htbp]

Systematic comparison of different architectural configurations revealed detailed performance characteristics across various KAN and MLP configurations, as shown in Table :

[htbp]


**Configuration** | **Parameters** | **Accuracy (%)** | **Time (s)**

{**KAN Configurations**}

KAN-32 (grid=5, spline=3) | 948,320 | 98.46 | 58.76
KAN-32 (grid=2, spline=2) | 684,512 | 98.44 | 49.78
KAN-4 (grid=2, spline=0) | 321,776 | 97.86 | 32.93

{**MLP Configurations**}

MLP (128, 64) | 560,173 | 98.46 | 30.10
MLP (64, 16) | 421,197 | 98.44 | 28.10
MLP (16, 4) | 321,717 | 96.05 | 28.82

Key findings from this comprehensive analysis include:

- Hyperparameter optimization proved more effective than scaling node parameters: reducing grid size (5→2) and spline order (3→2) achieved 15.3% speed improvement (58.76s→49.78s) with minimal accuracy loss (98.46%→98.44%)
- Node parameters showed minimal impact on testing time despite significant parameter count changes: KAN-32 configurations (948,320 parameters) required only 58.76s compared to KAN-4 configurations (321,776 parameters) at 32.93s, indicating that the 3× parameter increase resulted in only 78% time increase rather than the expected linear scaling, suggesting efficient parameter utilization in larger KAN architectures
- Larger MLP configurations (128×64, 64×16) achieved similar accuracy to KAN-32 configurations but with significantly lower computational overhead: MLP (128×64) achieved 98.46% accuracy in 30.10s with 560,173 parameters, while KAN-32 (grid=5, spline=3) achieved identical 98.46% accuracy in 58.76s with 948,320 parameters, demonstrating that MLPs require 41% fewer parameters and 49% less computation time for equivalent performance at larger scales

For direct architectural comparison with the same parameter counts, we conducted a controlled experiment comparing CNN+MLP and CNN+KAN configurations with nearly identical parameter counts, as shown in Table :

[htbp]


**Architecture** | **Test Accuracy** | **Test Time (s)** | **Parameters**

CNN+MLP | 96.05% | 28.82 | 321,717
CNN+KAN | 97.86% | 32.93 | 321,776

**Improvement** | **+1.81%** | **+14.3%** | **-**

This direct comparison reveals that:

- KAN achieved measurable accuracy improvement when parameter counts matched (321,776 vs 321,717 parameters)
- The additional 4.11s computational overhead (32.93s vs 28.82s) represents a reasonable trade-off for the 1.81% accuracy improvement in this application

### Hyperparameter Analysis and Ablation Studies

Systematic investigation of KAN hyperparameter impact across different configurations revealed distinct performance characteristics:

**KAN Hyperparameter Effects:** The following analysis was conducted to understand the impact of different KAN components:

- **Grid Size Effect:** Reduction from 5 to 2 achieved 15.3% speed improvement with minimal accuracy loss
- **Spline Order Impact:** Lower polynomial degrees (3→2→0) progressively reduced computational overhead
- **Node Scaling Analysis:** Hidden layer size variations (4-32 nodes) showed minimal effect on inference time but significant parameter count changes

**Parameter Efficiency Analysis:** Direct comparison of parameter types demonstrates that hyperparameter-induced parameters significantly affect computational overhead, while node parameters primarily influence model capacity with minimal latency impact.

## Discussion

### Cross-Domain Performance Analysis

Results demonstrate highly domain-dependent KAN effectiveness with distinct patterns across application domains:

**Computer Vision Success:** KANs showed clear advantages in chess piece classification, suggesting learnable activations benefit visual recognition tasks where subtle feature relationships are critical. The 1.81% accuracy improvement with matched parameters (97.86% vs 96.05%) represents a meaningful enhancement for the identical feature extraction pipeline, confirming superior representational capacity.

**Infrastructure Limitations:** Performance-critical applications like chess engines require mature optimization frameworks unavailable for novel architectures, creating practical deployment barriers. The migration from Burn to the Bullet framework was essential for NNUE success, highlighting infrastructure dependency for neural evaluation effectiveness.

**Optimization Strategy Insights:** KAN deployment should prioritize hyperparameter tuning over node scaling. Grid size and spline order reduction provide more effective efficiency gains than capacity reduction through hidden layer limitations, enabling practical deployment within computational constraints.

### Practical Deployment Guidelines

Based on experimental findings across two domains:

- **Computer Vision Applications:** KANs demonstrate potential where accuracy improvements justify computational overhead. The additional 4.11s testing time for processing the complete test dataset (20% of 150,000 augmented images, approximately 30,000 images) represents an acceptable trade-off for improved classification accuracy in applications prioritizing precision over speed
- **Real-time Systems:** Applications with strict latency constraints favor traditional architectures until KAN optimization matures. Performance-critical systems requiring millisecond-level responses should prioritize MLP architectures

### Research Implications

This work provides empirical evidence challenging assumptions about universal architectural advantages. The mixed results demonstrate that benefits are application-specific and infrastructure-dependent, emphasizing the importance of comprehensive evaluation in neural architecture research.

## Limitations and Future Work

The comprehensive empirical evaluation conducted in this research reveals both the significant potential of KAN architectures and the practical challenges that currently limit their broader adoption. Through systematic investigation across two application domains, we identified specific limitations that define clear research directions for advancing KAN capabilities.

### Current Limitations

- **Chess Engine Integration Barriers:** KAN chess engine evaluation remains unimplemented due to technical constraints, specifically the absence of pre-optimized KAN libraries suitable for high-performance applications, incompatibility with the Bullet framework's optimization infrastructure, and the complex mathematical operations in learnable activation functions requiring custom CUDA kernel development.

- **Computer Vision Scope Constraints:** Computer vision evaluation was limited to chess piece classification with a moderate dataset size (~10,000 images). While promising, these results may not generalize to other visual recognition tasks with different feature extraction requirements or larger-scale deployments.

- **Theoretical Analysis Gaps:** While practical performance characteristics were thoroughly documented, deeper theoretical analysis of KAN's learnable activation functions and their relationship to specific task characteristics remains limited.

- **Scope Limitations:** Initial exploration of KAN applications in chess behavioral analysis was attempted but proved inconclusive due to fundamental feature engineering challenges, highlighting the importance of domain-appropriate feature representation for meaningful architectural comparison.

### Future Directions

Priority areas for advancing KAN capabilities across identified domains:

- **Chess Engine Infrastructure:** This research revealed significant infrastructure barriers that must be addressed to enable meaningful KAN evaluation in chess engines:
  
    - Development of CUDA-optimized KAN kernels with efficient GPU acceleration capabilities
    - Framework integration with specialized chess engine libraries like Bullet
    - Comprehensive training pipeline development using chess-specific datasets
  

- **Computer Vision Optimization:** Expanding on the promising results in chess piece classification:
  
    - Scope expansion to evaluate KAN performance in diverse visual recognition tasks
    - Hardware-specific optimizations through custom GPU kernels
    - Neural architecture search and quantization techniques for KAN architectures
  

- **Theoretical Framework:** Advancing fundamental understanding of KAN properties:
  
    - Investigation of training efficiency and convergence characteristics
    - Interpretability analysis leveraging KAN's visualizable activation functions
    - Development of standardized benchmarking protocols for learnable activation functions
  

**Infrastructure Development:** Critical need for specialized optimization frameworks that can support novel architectures in production environments. This infrastructure gap represents a significant barrier to KAN adoption in demanding applications, requiring coordinated development efforts to establish the foundation for practical KAN deployment at scale .

## Conclusion

This research presents, to the best of our knowledge, the first comprehensive empirical evaluation of KANs across two practical domains, providing crucial insights into real-world applicability and performance characteristics. Results demonstrate domain-dependent effectiveness with clear advantages in computer vision tasks (1.81% accuracy improvement under matched parameter conditions) and significant infrastructure barriers in performance-critical applications (chess engines).

The study establishes a rigorous evaluation framework for neural architecture comparison and provides actionable guidance for practitioners considering KAN adoption. Key contributions include: (1) systematic comparison methodology enabling controlled architectural evaluation, (2) quantitative characterization of accuracy-efficiency trade-offs with specific optimization strategies, (3) identification of infrastructure requirements for practical KAN deployment, and (4) demonstration of KAN effectiveness in computer vision applications where learnable activation functions provide measurable advantages.

Critical findings reveal that KANs show promise in computer vision applications where learnable activation functions can capture subtle feature relationships, but successful deployment requires careful consideration of computational constraints. The 14.3% computational overhead proves acceptable for applications prioritizing accuracy over speed, while hyperparameter optimization (grid size and spline order tuning) provides more effective efficiency gains than capacity reduction.

Our findings contribute to realistic expectations for novel neural architectures and emphasize the importance of comprehensive empirical evaluation in advancing neural network research. The modular framework and methodology provide a foundation for future architectural comparison studies across diverse application domains, establishing evidence-based guidelines for practitioners navigating architectural choices in production systems.
