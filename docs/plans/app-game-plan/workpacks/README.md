# App + Game Workpacks

These workpacks execute the shared app/game evidence spine without creating
duplicate app and game systems. Each workpack must fill its AI worker checklist
before `DONE` or PR-ready reporting.

## Base Sequence

1. Contract boundary and Effect schemas.
2. Source index and doc reconciliation.
3. Current app/game snapshot and gap map.
4. App/game identity model.
5. Inventory evidence model.
6. Windows installed app/game inventory adapter.
7. Windows Store/UWP/AppX/MSIX inventory adapter.
8. Windows process runtime evidence adapter.
9. Windows foreground app/game evidence adapter.
10. Launcher evidence and game candidate model.
11. Cross-platform authority matrix.
12. App and game category/risk taxonomy.
13. Sessionization and duration engine.
14. Journal and SQLite ingest.
15. Read models and service events.
16. Parent portal app/game dashboard surfaces.
17. Unknown app/game approval flow.
18. Native game budgets and launcher policy.
19. Policy target compiler for app/game rules.
20. Time budget, schedule, and bonus-time integration.
21. Child-facing app/game warning and request UX.
22. Windows owned-process terminate time-limit proof.
23. Broad blocking proof gates.
24. AI classifier digest boundary.
25. Platform extension checklist and proof routing.
26. Install, uninstall, purchase, and store handoffs.
27. Performance and service health.
28. E2E, manual proof, rollout, and PR gate.
29. Rust protocol evidence identity parity.
30. Rust protocol authority classifier parity.
31. Journal/SQLite authority classifier storage.
32. Live process snapshot source.
33. Live process journal SQLite bridge.
34. Service capture app/game live process bridge.
35. Service app/game recurring freshness.
36. Live foreground window source.
37. Service foreground capture bridge.
38. Service authority/classifier surface evidence.
39. Authority/classifier read-model counts.
40. App/game boundary read-model event.
41. Live Windows inventory source.
42. Service Windows inventory capture bridge.
43. Live Windows Store package source.
44. Service Windows Store package capture bridge.
45. Live Windows registry inventory source.
46. Service Windows registry capture bridge.
47. Backend source freshness read-model rows.
48. WP49 category/risk policy routing.
49. WP51 app/game policy evaluator runtime breadth.
50. WP52 policy readiness service read model.
51. WP54 policy readiness portal renderer.
52. WP53 app/game notification intent contract.
53. WP56 app/game notification service read model.
54. WP58 app/game notification local outbox bridge.
55. WP59 app/game notification scheduler bridge.
56. WP60 app/game notification audit-history bridge.
57. WP61 app/game notification provider preflight.
58. WP62 app/game notification parent preference preflight boundary.
59. WP63 app/game source freshness source-panel intent seam.
60. WP64 app/game notification provider status handoff.
61. WP65 app/game notification parent preference status handoff.
62. WP66 app/game notification parent surface intent.
63. WP67 app/game notification parent surface renderer.
64. WP68 app/game notification live parent surface read model.
65. WP70 app/game policy preview handoff.
66. WP73 app/game platform extension proof-pack readiness.
67. WP74 app/game source freshness policy consumption.
68. WP75 app/game source freshness preview gate.
69. WP76 app/game source-gated policy preview read model.
70. WP78 app/game source-gated policy preview timer handoff.
71. WP79 app/game source-gated policy preview timer status.
72. WP81 app/game source-gated policy preview timer runtime readiness.
73. WP82 app/game source-gated policy preview timer scheduler persistence.
74. WP83 app/game source-gated policy preview timer audit rollback handoff.
75. WP84 app/game source-gated policy preview timer audit rollback read model.
76. WP85 app/game source-gated policy preview timer audit rollback parent surface intent.
77. WP86 app/game source-gated policy preview timer service readiness handoff.
78. WP87 app/game source-gated policy preview timer service readiness read model.
79. WP88 app/game source-gated policy preview timer service readiness protocol handoff.
80. WP89 app/game source-gated policy preview timer service readiness protocol read model.
81. WP90 app/game source-gated policy preview timer service readiness protocol command handoff.
82. WP91 app/game source-gated policy preview timer service readiness service handler handoff.
83. WP92 app/game source-gated policy preview timer service readiness read API handoff.
84. WP93 app/game source-gated policy preview timer service readiness read API response handoff.
85. WP94 app/game source-gated policy preview timer service readiness read API response consumer handoff.
86. WP95 app/game source-gated policy preview timer service readiness response consumer parent-surface handoff.
87. WP96 app/game source-gated policy preview timer service readiness response consumer parent-surface read-model handoff.
88. WP97 app/game source-gated policy preview timer service readiness response consumer parent-surface status handoff.
89. WP98 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model handoff.
90. WP99 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model parent-surface handoff.
91. WP100 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model parent-surface read-model handoff.
92. WP101 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model parent-surface read-model contract.
93. WP102 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model parent-surface read-model service handoff.
94. WP103 app/game source-gated policy preview timer service readiness response consumer parent-surface status read-model parent-surface read-model service read-model handoff.
95. WP104 app/game source-gated policy preview timer service event handoff.
96. WP105 app/game source-gated policy preview timer service read API handoff.
97. WP106 app/game source-gated policy preview timer service read API response handoff.
98. WP107 app/game source-gated policy preview timer service read API response consumer handoff.
99. WP108 app/game source-gated policy preview timer service read API response consumer parent-surface handoff.
100. WP109 app/game timer parent-surface service read-model command/event.
101. WP110 app/game timer parent-surface portal renderer.
102. WP111 app/game timer parent-surface active state-store bridge.
103. WP112 app/game timer parent-surface active-state portal visibility.
104. WP113 app/game timer parent-surface audit rollback active-state visibility.
105. WP114 app/game timer parent-surface control action-result visibility.
106. WP115 app/game timer parent-surface control result status visibility.
107. WP116 app/game timer parent-surface child reason/status refs.
108. WP117 app/game timer parent-surface child UX handoff readiness.
109. WP118 app/game timer parent-surface child UX local handoff artifact.
110. WP119 app/game timer parent-surface child UX local artifact visibility.
111. WP120 app/game timer parent-surface child UX local artifact records.
112. WP121 app/game timer parent-surface child UX local outbox bridge.
113. WP122 app/game timer parent-surface child UX local outbox scheduler bridge.
114. WP123 app/game timer parent-surface child UX local outbox provider preflight.
115. WP124 app/game timer parent-surface child UX local outbox provider status handoff.
116. WP125 app/game timer parent-surface child UX local outbox preference preflight.
117. WP126 app/game timer parent-surface child UX local outbox preference status handoff.
118. WP127 app/game timer parent-surface child UX local outbox parent surface intent.
119. WP128 app/game timer parent-surface child UX local outbox parent surface live visibility.
120. WP129 app/game timer parent-surface child UX local outbox parent surface live records.
121. WP130 app/game timer parent-surface child UX parent action cards.
122. WP131 app/game timer parent-surface child UX parent preference setup draft.
123. WP132 app/game timer parent-surface child UX parent preference setup read-only surface.
124. WP133 app/game timer parent-surface child UX parent preference setup service records.
125. WP134 app/game timer parent-surface child UX parent preference setup request boundary.
126. WP135 app/game timer parent-surface parent preference setup request action.
127. WP136 app/game timer parent-surface parent preference setup action-result handoff.
128. WP137 app/game timer parent-surface parent preference setup action-result persistence.
129. WP138 app/game timer parent-surface parent preference setup mutation receipt handoff.
130. WP139 app/game timer parent-surface parent preference setup child-runtime delivery handoff readiness.
131. WP140 app/game timer parent-surface parent preference setup child-runtime handoff command-result visibility.
132. WP141 app/game timer parent-surface parent preference setup child-runtime delivery local queue.
133. WP142 app/game timer parent-surface parent preference setup child-runtime delivery queue command-result visibility.
134. WP143 app/game timer parent-surface parent preference setup child-runtime delivery dispatch readiness.
135. WP144 app/game timer parent-surface parent preference setup child-runtime delivery dispatch command-result visibility.
136. WP145 app/game timer parent-surface parent preference setup child-runtime delivery receipt-required seam.
137. WP146 app/game timer parent-surface parent preference setup child-runtime delivery receipt-required command-result visibility.
138. WP147 app/game timer parent-surface parent preference setup child-runtime delivery receipt-pending seam.
139. WP148 app/game timer parent-surface parent preference setup child-runtime delivery receipt-pending command-result visibility.
140. WP149 app/game timer parent-surface parent preference setup child-runtime delivery receipt-ingested boundary.
141. WP150 app/game timer parent-surface parent preference setup durable local outbox runtime.
142. WP151 app/game timer parent-surface parent preference setup provider delivery readiness.
143. WP152 app/game timer parent-surface parent preference setup provider delivery manual-required handoff.
144. WP153 app/game timer parent-surface parent preference setup provider delivery preflight requirements.
145. WP154 app/game timer parent-surface parent preference setup provider preflight durable outbox.
146. WP155 app/game timer parent-surface parent preference setup provider delivery local queue.
147. WP156 app/game timer parent-surface parent preference setup provider delivery receipt tracking.
148. WP157 app/game timer parent-surface parent preference setup provider delivery receipt-ingested boundary.
149. WP158 app/game timer parent-surface parent preference setup provider delivery aggregate surface.
150. WP159 app/game malicious metadata safety gate.
151. WP160 app/game evidence boundary safety gates.
152. WP161 app/game enforcement platform no-claim safety gates.
153. WP162 app/game policy readiness parent-surface detail rows.
154. WP163 app/game dashboard boundary count visibility.
155. WP164 app/game dashboard readiness blocker cards.
156. WP165 app/game dashboard platform capability limitation rows.
157. WP166 app/game adapter execution readiness.
158. WP167 app/game adapter execution readiness live surface.
159. WP168 app/game adapter dispatch preflight live handoff.
160. WP169 app/game scoped adapter dispatch command-result.
161. WP170 app/game adapter dispatch execution audit.
162. WP171 app/game scoped adapter execution result bridge.
163. WP172 app/game scoped adapter execution store readback.
164. WP173 app/game scoped adapter dispatch execute command.
165. WP174 app/game scoped adapter dispatch executed parent surface.
166. WP175 app/game scoped adapter dispatch parent action surface.
167. WP176 app/game blocking time-limit done gate.
168. WP177 app/game category unknown policy readiness service consumption.
169. WP178 app/game adapter host capability state.
170. WP179 app/game platform manual artifact host probe refs.
171. WP180 app/game host capability summary counts.
172. WP181 app/game Android physical device proof.
173. WP182 app/game Linux WSL runtime proof.
174. WP183 app/game platform proof status surface.
175. WP184 app/game platform proof status service surface.
176. WP185 app/game Android UsageEvents foreground proof.
177. WP186 app/game Linux WSLg display readiness proof.
178. WP187 app/game platform proof status Android/Linux detail refs.
179. WP188 app/game Android UsageEvents replay readiness.
180. WP189 app/game Linux foreground capture readiness.
181. WP190 app/game Android UsageEvents child-runtime replay.
182. WP191 app/game Linux foreground source preflight.
183. WP192 app/game Android authority preflight.
184. WP193 app/game Windows broad blocking authority preflight.
185. WP194 app/game Android Accessibility overlay preflight.
186. WP195 app/game platform proof status preflight detail refs.
187. WP196 app/game Apple CI platform proof preflight.
188. WP197 app/game Linux Docker host preflight.
189. WP198 app/game Android UsageEvents capability proof.
190. WP199 app/game Android UsageEvents runtime preflight.
191. WP200 app/game Android UsageEvents count sample.
192. WP201 app/game Android UsageEvents package runtime proof.
193. WP202 app/game Android Accessibility runtime proof.
194. WP203 app/game Windows local policy evidence proof.
195. WP204 app/game Linux active-window tool proof.
196. WP205 app/game platform proof status runtime detail refs.
197. WP206 app/game child-device delivery readiness.
198. WP207 app/game child-device runtime writer.
199. WP208 app/game child runtime transport receipt boundary.
200. WP209 app/game child runtime transport receipt service.
201. WP210 app/game child runtime transport receipt parent surface.
202. WP211 app/game Android child runtime transport receipt proof.
203. WP212 app/game Android child runtime local receipt proof.
204. WP213 app/game Android child runtime local receipt physical proof.
205. WP214 app/game Android child runtime local receipt ack proof.
206. WP215 app/game Android child runtime local receipt channel proof.
207. WP216 app/game Android child runtime local delivery intake proof.
208. WP217 app/game Android child runtime local delivery queue proof.
209. WP218 app/game Android UsageEvents granted sample proof.
210. WP219 app/game Android Accessibility enabled sample proof.
211. WP220 app/game Android child runtime local notification proof.
212. WP221 app/game Android child runtime local notification action proof.
213. WP222 app/game Android child runtime local notification request queue proof.

## Standard AI Worker Checklist

- [ ] Confirm source docs read: folder README, source index, current snapshot,
      shared evidence spine plan, native apps slice, native games slice,
      platform deep dive, test blueprint, UI/UX guide, main checklist, and this
      workpack.
- [ ] Confirm browser-game scope remains in browser-plan.
- [ ] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [ ] Before-state source snapshot recorded in
      `output/app-game-plan-proof/<workpack-id>/00-source-snapshot.md`.
- [ ] Contracts updated first where behavior changes.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist.
- [ ] Tests/proof listed in this workpack and test blueprint are implemented or
      explicitly manual-required with reason.
- [ ] Security/no-claim negative proof captured where applicable: inventory is
      not use, runtime is not foreground, foreground is not content, launcher is
      not game, AI cannot enforce, manual-required cannot execute.
- [ ] Feature/expectation/product-checklist/README update decision recorded.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before
      `DONE`.
