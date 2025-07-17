
# Whitepaper de Asami Club

Este whitepaper describe el sistema y ecosistema según lo proyectado y fue revisado por última vez en julio de 2025.

---

## 1. Panorama general

En Web3, donde la apertura, la descentralización y la comunidad son valores fundamentales, difundir ideas mediante campañas mediáticas de arriba hacia abajo resulta costoso, suele generar desconfianza y, en última instancia, es ineficaz para la adopción sostenida de usuarios. Aunque la publicidad tradicional puede generar visibilidad a corto plazo, rara vez construye confianza o genera sentido de pertenencia. La mayoría del crecimiento significativo en este espacio ha venido de usuarios tempranos y divulgadores: personas que comparten lo que han descubierto por curiosidad, convicción o ganas de ayudar a otros.

A estas personas, las llamamos **Divulgadores**. Actúan como curadores, intérpretes y amplificadores. Repostean, traducen y contextualizan proyectos emergentes. Aportan visibilidad, legitimidad y tracción inicial. Pero como sus contribuciones son informales y, a menudo, no remuneradas, suelen pasar desapercibidas. Muchos se alejan. Algunos son absorbidos por ecosistemas que ofrecen mayor reconocimiento. Otros simplemente se rinden.

No porque estuvieran equivocados — sino porque el ecosistema no les dio motivos para continuar.

### Una Brecha Estructural

Los proyectos Web3 sí se preocupan por la visibilidad, pero las herramientas disponibles son limitadas. La mayoría de los presupuestos de marketing terminan:

* En plataformas que están **desalineadas con los valores de Web3**, o que directamente restringen el contenido relacionado con blockchain.
* O en **esquemas de uso incentivado**, donde se paga a individuos por completar tareas, registrar cuentas o simular interacción — atrayendo con frecuencia usuarios que desaparecen en cuanto se acaban las recompensas.

Lo que falta en estos enfoques es una forma de reconocer y apoyar la **divulgación genuina** — no el uso, no la interacción artificial, sino la expresión pública de interés y convicción.

Los proyectos de código abierto y descentralizados tienen éxito cuando encuentran y retienen a **divulgadores voluntarios**. Pero la mayoría no tiene una forma sistemática de identificarlos, agradecerles o motivarlos a seguir.

### El Protocolo Asami

**Asami.club es un protocolo descentralizado para reconocer y recompensar a divulgadores de Web3.** Ayuda a los Proyectos a:

* Alcanzar personas reales que los apoyan públicamente,
* Agradecerles de forma transparente,
* Y construir una red duradera de voces alineadas.

Antes de participar en cualquier campaña, cada usuario es evaluado y clasificado por Asami mediante un **proceso periódico de medición de influencia**. Esta evaluación de base considera impresiones en tweets, patrones de interacción, proporción de seguidores verificados, idioma y afinidad con Web3. Solo quienes superan un umbral de calidad son admitidos en el grupo de Divulgadores elegibles.

Una vez curados dentro de este grupo, los Divulgadores pueden **descubrir campañas y sumarse voluntariamente** al repostear contenido en el que creen. Los Proyectos crean campañas seleccionando una publicación (usualmente un tweet) y asignando un presupuesto en DOC, una stablecoin respaldada por Bitcoin.

Cuando un Divulgador elegible hace repost, puede recibir:

* Una modesta recompensa en DOC, como agradecimiento público.
* Tokens ASAMI, que otorgan participación en ingresos del protocolo y derechos de gobernanza.

Este pago **no es un incentivo para repostear**, sino una forma de **reconocer y valorar** la visibilidad que ya ofrecieron. También funciona como un **símbolo de responsabilidad**: al aceptar una recompensa pública por una acción pública, el Divulgador demuestra transparencia y apropiación moral del mensaje que amplificó.

A partir de ahí, **los Proyectos pueden comenzar a curar su propia comunidad de divulgadores**. Pueden observar quiénes participaron, mantener listas blancas y negras, y crear campañas totalmente abiertas, semirrestrictas o exclusivas para un grupo de confianza. Esto les permite equilibrar la **exploración** (encontrar nuevos posibles Divulgadores) con la **explotación** (colaborar con voces ya probadas). Algunos proyectos incluso pueden continuar la relación por fuera de Asami para colaboraciones más profundas, como creación de contenido o programas de embajadores.

### Beneficios para el Ecosistema

Asami es una posible solución al desafío de hacer crecer Web3 a través de la confianza, no del ruido. A diferencia de las plataformas que pagan por usar un producto o realizar tareas arbitrarias, Asami se enfoca en **reconocer la divulgación genuina y voluntaria**.

Proporciona una estructura para:

* Motivar a los Divulgadores a seguir compartiendo lo que ya les importa.
* Ayudar a los Proyectos a descubrir y valorar a estos primeros aliados.
* Mantener la interacción pública, modesta y beneficiosa para ambas partes — sin comprometer la independencia ni la integridad.

Como todas las interacciones están en cadena y la participación no requiere permisos, no hay contratos, acuerdos privados ni influencia oculta. Y dado que cada campaña ayuda a los Proyectos a identificar voces alineadas, el resultado no es solo visibilidad — es **formación de comunidad**.

Esto crea una **economía circular de reputación y apoyo**: los Proyectos financian campañas, los Divulgadores reciben reconocimiento, y ambas partes ayudan a que el ecosistema Web3 crezca en coherencia con sus valores.

---

## 2. Historia del Proyecto, Descubrimientos y Estado Actual

En 2023, **Nostr** estaba reinventando las redes sociales como una plataforma descentralizada — construida sobre criptografía simple y participación sin permisos. Estaba siendo promovida por defensores apasionados, al igual que otros movimientos anteriores, como Bitcoin. Lo que distinguía a Nostr era que no era simplemente un protocolo neutral — apoyaba activamente a sus primeros usuarios mediante **señales sociales y micro-recompensas**, como likes y zaps.

Esto dio origen a la idea original detrás de Asami: ¿y si combinamos esta cultura de **divulgación social** con **contratos inteligentes**, permitiendo que las personas sean recompensadas — no por usar algo, sino por amplificarlo? Si alguien decidía volver a publicar un mensaje en Nostr, un contrato inteligente podía verificar esa acción y enviarle automáticamente una recompensa. Sin intermediarios publicitarios centralizados, sin negociaciones — solo un agradecimiento sin necesidad de confianza.

### 2.1 De Experimentos de Protocolo a Divulgación Práctica

El primer prototipo de Asami implementó exactamente esta idea: un contrato inteligente que recibía fondos de un Proyecto y permitía que cualquier persona reclamara una recompensa por volver a publicar un mensaje en Nostr. Aunque técnicamente elegante y completamente descentralizado, surgieron dos limitaciones críticas:

* Verificar criptográficamente las publicaciones en cadena resultaba **demasiado costoso para escalar**, especialmente para micropagos.
* La **base de usuarios de Nostr era aún demasiado pequeña** para atraer interés significativo por parte de los Proyectos.

Esto condujo a un giro: Asami se integró con **X (antes Twitter)**, adoptando un modelo híbrido. Las publicaciones ahora se verifican fuera de cadena por un tercero — el Administrador de Campañas — y luego se registran en cadena. Esto redujo los costos y permitió un mayor alcance, sin comprometer los valores del protocolo de transparencia y trazabilidad.

### 2.2 Supuestos de Mercado y Lo que Aprendimos

Asami se construyó sobre dos suposiciones clave:

* Que **los usuarios de Web3 estarían dispuestos a volver a publicar contenido en el que creen**, si se sienten respetados y reconocidos.
* Que **los Proyectos de Web3 preferirían visibilidad auténtica de verdaderos Divulgadores**, en lugar de métricas infladas o interacción falsa.

Ambas se demostraron ciertas — pero la ejecución requería matices.

Los Divulgadores se sumaron con entusiasmo, a menudo republicando sin esperar grandes pagos. Los Proyectos valoraron ver apoyo orgánico. Pero rápidamente quedó claro que **un sistema de recompensas justo y estable** requería:

* Medición precisa de la influencia,
* Expectativas saludables sobre precios,
* Y mecanismos para que los Proyectos **construyan sus propias comunidades**, no solo alquilen atención.

También se volvió evidente que el objetivo nunca fue “pagar para que la gente publique”. En cambio, la recompensa debía enmarcarse — y diseñarse — como un **agradecimiento**, entregado de forma pública, modesta, y solo después del acto de divulgación.

### 2.3 Desafíos Iniciales y Soluciones

Varias fricciones tempranas moldearon la evolución del protocolo:

* **Puntajes poco confiables**: Los primeros modelos basados en la cantidad de seguidores eran fáciles de explotar. El algoritmo actual de medición de influencia usa impresiones recientes de tweets, ratios de interacción, menciones, proporciones entre seguidores verificados y seguidos, y datos opcionales ingresados manualmente. Los puntajes se actualizan periódicamente, de forma independiente a la participación en campañas. Esto crea un filtro base que asegura que solo cuentas auténticas y relevantes puedan participar en campañas.

* **Crecimiento limitado de divulgadores**: Un número pequeño de Divulgadores tempranos concentraba la mayoría de las recompensas. Para alentar nuevas voces, Asami añadió **incentivos por referidos** y hace seguimiento de la participación a largo plazo.

* **Fricciones en la creación de campañas**: Aunque los Divulgadores podían recibir cripto fácilmente, muchos Proyectos — especialmente aquellos con equipos de marketing nativos de Web2 — preferían usar tarjetas de crédito. Para apoyarlos, Asami integró **pagos con Stripe**, permitiendo financiar campañas sin usar cripto. (Se aplica una comisión del 20% para cubrir costos y slippage.)

* **De alcance a relación**: Los Proyectos no solo querían impresiones — querían conexión. Las campañas comenzaron a funcionar como herramientas de descubrimiento: formas de encontrar posibles Divulgadores, observarlos en acción, y luego invitarlos a listas de permitidos para campañas futuras. Los Proyectos ahora equilibran **exploración** (medir interés) con **explotación** (trabajar con voces conocidas), y pueden contactar a Divulgadores confiables fuera de la plataforma si lo desean.

### 2.4 Enfoque Exclusivo en Web3 y Visión a Largo Plazo

Originalmente, Asami dejaba abierta la posibilidad de un uso más amplio. Pero con el tiempo quedó claro que **los Proyectos Web3 están mejor alineados** con los valores del protocolo: transparencia, descentralización y crecimiento basado en comunidad.

Asami ahora se enfoca exclusivamente en el ecosistema Web3. Su misión es ayudar a los Proyectos a:

* Llegar a Divulgadores alineados,
* Reconocerlos de manera justa,
* Y hacer crecer sus comunidades de forma sostenible.

A mediados de 2025, el sistema está en funcionamiento y plenamente operativo. Las campañas corren a diario. El sistema de puntajes está activo. Los Divulgadores se registran con X o correo electrónico, y vinculan sus billeteras cuando están listos. El onboarding con Stripe funciona. Los retiros sin gas están disponibles. El protocolo es de código abierto, evoluciona continuamente, y da la bienvenida a nuevos Administradores de Campaña e integraciones.

Para reforzar el tono de **agradecimiento en lugar de compensación**, Asami adoptó al **maní** como su mascota — un recordatorio lúdico de que las recompensas pueden ser modestas, pero son sinceras. Lo que los Divulgadores aportan es visibilidad, no riqueza. Los tokens DOC y ASAMI que reciben son simplemente **un agradecimiento público**.

Mirando hacia el futuro, Asami se enfocará en:

* Avanzar el marco técnico y filosófico para medir la influencia genuina,
* Expandirse a nuevas plataformas y tipos de interacción (por ejemplo, comentarios, likes, LinkedIn, Nostr mediante oráculos),
* Y habilitar **puntajes por acción**, donde los Divulgadores puedan ser valorados por el alcance de un post o quote tweet específico, no solo por su cuenta general.
* Soportar pagos con Bitcoin y Lightning Network (LN) para financiar campañas, con conversión automática a DOC y comisiones más bajas que las de tarjeta de crédito — ampliando la accesibilidad sin comprometer la integridad de las campañas.

## 3. Roles del Ecosistema y Partes Interesadas

El protocolo Asami reúne tres roles principales:

* **Proyectos**, que desean aumentar su visibilidad y construir una red de seguidores confiables.
* **Divulgadores**, que amplifican mensajes en los que creen y reciben reconocimiento público por hacerlo.
* **Administradores de Campaña**, que operan la infraestructura y hacen cumplir reglas a nivel de protocolo como el registro de reposts y el cálculo de influencia.

Cada participante cumple un rol específico en el sistema, con derechos, responsabilidades y expectativas claramente definidas.


### 3.1 Proyectos

**Quiénes son:** Cualquier persona o equipo que dirija una iniciativa Web3 — protocolos, dApps, DAOs, proyectos de herramientas, artistas, proveedores de infraestructura o movimientos impulsados por la comunidad — que quiera involucrar a personas reales para amplificar su mensaje.

**Qué hacen:** Los Proyectos crean campañas seleccionando una publicación (usualmente en X), definiendo un presupuesto en DOC y eligiendo un Administrador de Campaña. Luego pueden establecer parámetros sobre cómo se distribuyen las recompensas y quién puede participar.

**Derechos:**

* Definir el presupuesto, duración y estructura de pagos de la campaña.
* Restringir o permitir la participación mediante listas blancas o negras.
* Revisar participantes y crear una lista evolutiva de Divulgadores confiables.
* Financiar campañas usando DOC, tarjeta de crédito (vía Stripe), o pronto, Bitcoin y LN.
* Elegir y cambiar de Administrador de Campaña libremente.
* Retirar fondos no utilizados una vez finalizada la campaña.

**Obligaciones:**

* Aceptar que las campañas, una vez activas, no se pueden revertir.
* Confiar en que el Administrador de Campaña aplicará el registro de reposts y la puntuación de influencia de forma justa.

**Notas:**

* Los Proyectos pueden crear campañas abiertas para descubrir nuevos Divulgadores (exploración), o exclusivas para comunidades previamente seleccionadas (explotación).
* Los reposts son voluntarios — los Proyectos no pueden obligar a nadie a amplificar su contenido.
* Todos los pagos son públicos y trazables en la cadena.


### 3.2 Divulgadores

**Quiénes son:** Usuarios de redes sociales — principalmente en X — que tienen una audiencia genuina y eligen amplificar responsablemente proyectos Web3.

Los Divulgadores no tienen que ser celebridades, fundadores ni personas influyentes del sector. Cada voz auténtica importa. Algunos Divulgadores pueden estar recién comenzando, compartiendo nuevos proyectos con amigos, compañeros de trabajo o una audiencia pequeña pero real. Eso está bien. La divulgación no se trata de hacer ruido, sino de estar alineado con lo que se comparte.

**Qué hacen:** Los Divulgadores exploran campañas disponibles (tras superar el puntaje base de Asami) y eligen qué contenidos repostear. Si su repost cumple con los requisitos de la campaña, reciben un pago modesto en DOC y tokens ASAMI.

**Qué define a un verdadero Divulgador:**
Asami define a un Divulgador como alguien que:

* Posee y hace crecer una **audiencia genuina** en X.
* **Comparte contenido Web3 de forma responsable**, expresando interés o conocimiento real.
* **No repostean a ciegas** ni solamente para obtener recompensas.
* Brindan **visibilidad real** a los proyectos que amplifican.

Esto significa que hacer reposts no es suficiente: se espera que los Divulgadores muestren interés, comprensión y relevancia.


**Derechos:**

* Elegir qué campañas repostear — la participación siempre es voluntaria.
* Ser recompensado (en DOC y ASAMI) si su repost es válido y tiene impacto.
* Solicitar una revisión de su puntaje de influencia o de sus etiquetas asignadas (por ejemplo, idioma o región).
* Usar retiros sin gas para reclamar sus recompensas.

**Obligaciones:**

* Mantener los reposts durante un período mínimo.
* Operar una cuenta legítima en X con actividad orgánica.
* Evitar repostear todas las campañas sin contexto o criterio.
* Aceptar que su puntaje puede cambiar con el tiempo, y que no todos los cambios generarán pagos.

**Sobre el puntaje e impacto:**

El sistema de puntaje de influencia de Asami es imperfecto por diseño. Todavía estamos descubriendo cómo medir mejor la influencia real. Si estás comenzando tu camino como Divulgador y tu puntaje es bajo, no te desanimes. Sigue aprendiendo, opinando y compartiendo tus ideas online. Los quote tweets, las opiniones honestas, las críticas constructivas y la participación auténtica cuentan. Evita el contenido generado por IA o los trucos — suelen reducir el alcance real y restan valor a largo plazo.

Algunas personas pueden intentar mejorar su puntaje como una forma de reconocimiento — y eso está bien. Pero si el puntaje se vuelve una obsesión o se ve como una fuente de ingresos, probablemente llegue la decepción. Las recompensas en Asami no están garantizadas ni fueron diseñadas para sostener un ingreso económico.

Sabemos que nuestro algoritmo tal vez no reconozca del todo tu valor. Si eso sucede, no significa que no tengas influencia — tal vez aún no contamos con las herramientas adecuadas para medir el tipo de influencia que tú aportas. Con el tiempo, a medida que tu audiencia y nuestras herramientas mejoren, esperamos que tu puntaje refleje tu esfuerzo.

**Reclamos y revisiones:**
Los Divulgadores pueden solicitar una revisión de su puntaje, pero deben:

* Referenciar partes específicas de los criterios de puntuación.
* Comparar su actividad con la de otros perfiles similares.
* Proveer enlaces o métricas de respaldo (por ejemplo, estadísticas de engagement, referidos).
* Incluir el enlace a su perfil público de Asami.

Quejas generales como “mi puntaje es muy bajo” sin evidencia pueden ser ignoradas.


### 3.3 Administradores de campañas

**Quiénes son:** Operadores independientes que ejecutan el software del protocolo Asami, procesan los reposts, calculan la influencia, gestionan etiquetas de usuario y aplican las reglas de las campañas.

**Qué hacen:**

* Detectan y registran los reposts.
* Calculan la influencia usando su propia lógica.
* Manejan los retiros sin gas y la atención al usuario.
* Administran listas de acceso y bloqueo en nombre de los Proyectos.
* Opcionalmente ofrecen servicios como estrategia de contenido o análisis.

**Derechos:**

* Definir y actualizar su propia lógica de puntuación de influencia.
* Decidir qué Divulgadores son elegibles para participar en campañas.
* Mantener rankings públicos, etiquetas y etiquetas de engagement.
* Establecer sus propios honorarios (adicionales a las tarifas del protocolo, si lo desean).

**Obligaciones:**

* Usar lógica clara y reproducible.
* Responder a consultas bien formuladas de los usuarios.
* Operar con transparencia — toda lógica de campaña debe estar abierta a auditoría.
* Respetar las reglas de campaña definidas por cada Proyecto.

**Límites:**

* No pueden revertir pagos una vez realizados.
* No pueden modificar la lógica del contrato inteligente.
* Pueden ser reemplazados en cualquier momento — Proyectos y Divulgadores pueden elegir otros administradores.

Asami no otorga ningún privilegio especial a los Administradores de campañas. Su autoridad proviene de la calidad de su servicio y su capacidad para generar confianza. Si no actúan con transparencia, o si su sistema de puntuación es irrazonable, cualquiera puede lanzar administradores alternativos en cualquier momento.

### 3.4 Titulares del Token

**Quiénes son:** Cualquier persona que posea tokens ASAMI — incluidos Divulgadores, Proyectos y Administradores de campañas que hayan recibido tokens por participar en campañas o mediante compra directa.

**Qué hacen:** Los titulares de tokens ASAMI se benefician de una alineación a largo plazo con el protocolo. Reciben una parte de las tarifas cobradas y participan en la gobernanza a nivel de protocolo.

**Derechos:**

* Recibir una parte proporcional de las tarifas en DOC retenidas por el protocolo.
* Votar sobre la tasa de tarifa del protocolo mediante un mecanismo de promedio ponderado.
* Poseer, transferir o vender tokens ASAMI libremente.

**Obligaciones:**

* Ninguna impuesta por el protocolo. Sin embargo, se espera gobernanza responsable de quienes se preocupan por el futuro de Asami.

**Notas:**

* Poseer tokens ASAMI no otorga poder sobre decisiones de campañas ni sobre el sistema de puntuación.
* Los titulares de tokens **no controlan** a los Administradores de campañas ni influyen en los resultados individuales de campañas.
* El único mecanismo de gobernanza compartido actualmente en funcionamiento es la posibilidad de votar sobre la **tasa de tarifa** a nivel del protocolo, que afecta la distribución de ingresos y la emisión de tokens.

> En la práctica, esto significa que los titulares de tokens ayudan a equilibrar dos objetivos:
>
> * Tarifas más altas aumentan la cantidad de DOC distribuido a los titulares, pero también inflan la oferta de tokens y pueden reducir la participación en campañas.
> * Tarifas más bajas hacen crecer el ecosistema y reducen la inflación, pero disminuyen los retornos a corto plazo.

La gobernanza es por lo tanto mínima, orientada a incentivos y transparente. Tener tokens es una forma de **participar en el futuro del club**, no una herramienta para controlar a los demás.

---

## 4. Medición de Influencia

En Asami, la influencia determina si un Divulgador puede participar en campañas y cuánto será recompensado. Pero a diferencia del conteo de seguidores o los perfiles verificados, Asami trata la influencia como un modelo funcional — **una estimación dinámica basada en datos** que busca reflejar tanto el alcance como la credibilidad.

El sistema no pretende medir la “influencia real” de forma perfecta. En cambio, utiliza los datos públicos disponibles para hacer la mejor aproximación posible en un momento determinado — y mejora con el tiempo. Las puntuaciones de influencia se recalculan de forma **periódica** (actualmente cada 20 días), independientemente de la actividad en campañas. Esto asegura equidad y evita manipulaciones rápidas o artificiales.

La influencia de cada Divulgador se define por la combinación de:

* **Tamaño de audiencia** — cuántas personas probablemente ven sus publicaciones
* **Autoridad** — cuán significativa o creíble es su voz

Estas dos puntuaciones se **multiplican** para producir una Puntuación de Influencia final, que determina tanto la elegibilidad como el tamaño de la recompensa en las campañas. La **última y próxima fecha de puntuación** de cada usuario se muestra en su perfil público de Asami.


### 4.1 Tamaño de Audiencia

El Tamaño de Audiencia mide cuántas personas probablemente ven el contenido de un usuario en X — no basado en seguidores, sino en impresiones reales de sus tweets.

Para calcular el Tamaño de Audiencia, Asami:

* Recoge **todas las publicaciones y respuestas públicas** de los últimos 30 días
* Calcula el **promedio de impresiones** de esas publicaciones
* Calcula la **mediana de impresiones** del mismo conjunto
* Luego **promedia esos dos valores** para producir un Tamaño de Audiencia final

Este método híbrido reduce el efecto de valores atípicos (como un tweet viral aislado), al tiempo que recompensa la actividad constante. Al incluir **respuestas**, el sistema captura tanto los estilos de publicación de perfil como los conversacionales — pero con algunas salvedades.

> 📝 **Nota sobre las respuestas:** Las respuestas tienden a recibir menos impresiones que las publicaciones originales. Así que los Divulgadores que se apoyan fuertemente en respuestas pueden mostrar un Tamaño de Audiencia más bajo, incluso si están activos. Este comportamiento podría ajustarse en futuras versiones del algoritmo. Sin embargo, si las respuestas de un usuario generan buena interacción, su **puntuación de autoridad** probablemente lo reflejará — por lo tanto, el compromiso reflexivo sigue siendo recompensado.

El Tamaño de Audiencia siempre se calcula de forma pesimista — si no se puede demostrar visibilidad real a través de métricas públicas, no se asume ninguna audiencia.

### 4.2 Autoridad

La Autoridad mide **cuánto peso tiene tu voz ante tu audiencia**. No basta con ser visto — el sistema también busca señales de que las personas prestan atención, interactúan y posiblemente confían en lo que compartes.

La Autoridad se calcula mediante una evaluación en capas de varias señales públicas. Si no se detecta compromiso significativo, la Autoridad se establece en 0%, y el usuario queda inhabilitado para recibir recompensas de campaña — sin importar su Tamaño de Audiencia.

Cada uno de los criterios a continuación suma o modifica la puntuación. El resultado es un porcentaje entre 0% y 100%, que se multiplica por el Tamaño de Audiencia para producir la Puntuación de Influencia final.

#### Interacción Recibida en X

Este es el **criterio central**. Si las publicaciones de un usuario consistentemente no reciben interacción significativa de otros, no se considera que tenga influencia.

Asami evalúa:

* La proporción de **me gusta, respuestas, comentarios, citas y retweets** respecto a las impresiones
* Si el usuario es **mencionado por otros**, especialmente en **publicaciones destacadas**
* Su **estado de verificación** (por ejemplo, X Premium, insignia heredada)
* La cantidad de **seguidores verificados**, en relación con el total de seguidores

Un número alto de impresiones con poca o nula interacción resultará en **baja autoridad**, y por tanto, una baja Puntuación de Influencia.

Niveles de puntuación:

* **Nulo**: Las publicaciones son ignoradas o la interacción es artificial → 0%
* **Promedio**: Interacción constante y orgánica → +25%
* **Alto**: Fuerte compromiso, interés público → +50%

#### Encuestas Directas a la Audiencia

Los Divulgadores pueden realizar encuestas anónimas para preguntar a sus seguidores cuánto confían en sus recomendaciones. Estas encuestas brindan información sobre la autoridad percibida.

Ejemplo de pregunta:

> *“Si recomiendo un proyecto, ¿tú normalmente: (a) Lo sigues sin pensar, (b) Lo consideras, (c) Lo ignoras, (d) Haces lo contrario?”*

Niveles de puntuación:

* **Nulo**: Sin encuestas recientes → +0%
* **Promedio**: Respuestas mixtas, algo de confianza → +10%
* **Alto**: Consenso positivo claro → +20%
* **Reverso**: La mayoría dice que haría lo contrario → reduce a la mitad la puntuación de Interacción

#### Interacción Fuera de X

Algunas personas tienen influencia significativa en otros entornos (por ejemplo, podcasts, conferencias, encuentros, DAOs). Este criterio considera la **reputación fuera de la plataforma**.

Niveles de puntuación:

* **Nulo**: Sin presencia notable fuera de X → +0%
* **Promedio**: Activo en comunidades relacionadas → +5%
* **Alto**: Voz reconocida en círculos Web3 → +10%

La evidencia puede ser enviada manualmente durante una revisión.
#### Estado en X

El estado funcional de una cuenta importa.

Reglas de puntuación:

* **Suspendida, restringida o shadowbanned**: Autoridad = 0%
* **Cuenta operativa normal**: Sin cambio
* **Verificada / Premium**: +10%

#### Autoridad por Referencias

Los Divulgadores que refieren a otros a Asami y los ayudan a tener éxito demuestran influencia.

Regla de puntuación:

* **Referencias exitosas de cuentas activas** → +10%

#### Comportamiento de Tenencia de Tokens

Los Divulgadores que **mantienen tokens ASAMI** en lugar de venderlos inmediatamente demuestran alineación con el proyecto y su éxito a largo plazo.

Regla de puntuación:

* **Tener tokens durante el período de evaluación** → +10%

#### Cálculo Final

Para calcular la Autoridad:

1. Comenzar con la base de **Interacción Recibida en X**

   * Si es 0%, se omiten todos los demás factores.
2. Sumar (o modificar) el resto:

   * Encuestas a la Audiencia
   * Presencia fuera de X
   * Estado de la Cuenta en X
   * Referencias
   * Tenencia de Tokens

Autoridad máxima posible: **100%**
Mínima: **0%** (sin interacción o con señales de inelegibilidad)

> 🎯 **Nota:** El sistema está diseñado para recompensar comportamientos que generan confianza real — no métricas de vanidad. Comprar likes, repostear sin criterio o generar interacciones superficiales no mejorará tu puntuación. La autoridad se gana lento y se pierde rápido.

### 4.3 Puntuación Final de Influencia

Una vez que se calculan el **Tamaño de Audiencia** y la **Autoridad** de un Divulgador, se **multiplican** para producir su **Puntuación de Influencia** final.

Esta puntuación se usa para:

* Determinar si puede participar en campañas
* Asignar recompensas en DOC y tokens ASAMI
* Ranquear a los Divulgadores en los rankings públicos

La Puntuación de Influencia se actualiza **periódicamente** (actualmente cada 20 días), de forma independiente a las campañas. Esto asegura que todos los participantes sean evaluados con base en su comportamiento reciente y que nuevos Divulgadores puedan calificar sin tener que esperar campañas específicas.

Este modelo recompensa tanto el alcance como la confianza. Una gran audiencia sin interacción genera una puntuación débil. Una voz creíble sin audiencia también. Los Divulgadores más efectivos son los que son **vistos y respetados** — y, idealmente, **consistentes en el tiempo**.


### 4.4 Una Nota sobre Transparencia y Límites

Asami no pretende medir la influencia perfectamente. La influencia es contextual, evoluciona y es difícil de cuantificar — especialmente cuando los datos son incompletos o la actividad es sutil.

Algunos Divulgadores pueden sentirse subestimados, especialmente si su valor radica en redes silenciosas, interacciones privadas o influencia fuera de la plataforma que Asami aún no puede detectar.

Si eres uno de ellos:

* Puedes solicitar una revisión de tu puntuación siguiendo el proceso público estándar.
* Pero no tomes tu puntuación actual como un juicio sobre tu valor, potencial o integridad.

> **Tu camino y el nuestro pueden encontrarse en el destino.**
> Si sigues construyendo una audiencia orgánica, participando con criterio y divulgando con sinceridad — y mientras seguimos mejorando nuestro algoritmo para detectar mejor la influencia genuina — es muy probable que eventualmente nos alineemos.

No intentamos capturar todos los tipos de influencia — solo aquellos que podemos medir con confianza, de forma justa y reproducible. A medida que nuestras herramientas mejoren, esperamos poder reconocer a más Divulgadores en más contextos.

Hasta entonces, recuerda: la puntuación de Asami es solo eso, una puntuación. Tu voz es tuya.

---

## 5. Tokenómica y Distribución de ASAMI

El **token ASAMI** es el activo nativo del protocolo Asami.club. Fue diseñado para proporcionar **equidad y participación en el crecimiento** a quienes hacen crecer el club: **Divulgadores**, **Administradores de Campaña** y **Proyectos**.

A diferencia de plataformas que dependen de financiación anticipada o ventas de tokens, Asami solo emite tokens cuando hay **uso real** del protocolo. El token ASAMI alinea los incentivos en torno a la participación sostenida y la divulgación responsable — no la especulación.

Todos los datos y estadísticas relacionados con el token están disponibles públicamente en [asami.club/#/Token](https://asami.club/#/Token).


### 5.1 Participación en Ingresos e Incentivos

El protocolo Asami redistribuye sus ingresos a las personas que lo hacen crecer — no solo mediante la emisión de tokens, sino también a través de un sistema de **participación en ingresos** vinculado a la tenencia de tokens.

Cada ciclo de 15 días, el **pool de comisiones del protocolo** — recaudado de todas las colaboraciones pagas — se distribuye entre los tenedores de ASAMI elegibles. Esto crea un incentivo para **mantener tokens** y **participar a largo plazo**, en lugar de especular o revender.

#### Reglas de Elegibilidad

Para recibir tu parte del pool de ingresos, debes:

* ✅ **Tener al menos 4,000 tokens ASAMI**
* ✅ **No mover tus tokens durante el ciclo de 15 días en curso**
* ✅ **Haber reclamado tus tokens a una wallet personal (no un contrato)**

Los ciclos de tokens están atados a los **períodos definidos por el contrato inteligente**, no a ventanas móviles. El rango exacto de fechas de cada ciclo está visible en la [página de estadísticas del token](https://asami.club/#/Token).

> ⛔ **Importante:**
>
> * Si **mueves tus tokens** (los transfieres, vendes, stakeas o envías de cualquier forma) durante un período, **pierdes elegibilidad** para las recompensas de ese ciclo.
> * Si no has **reclamado tus tokens**, no cuentan. Solo los saldos reclamados son elegibles.
> * Los tokens ASAMI **en contratos** — incluyendo pools de liquidez — **no** reciben ingresos a menos que su código lo habilite explícitamente.

#### ¿Qué pasa con las partes no reclamadas?

Cuando una persona es elegible pero **no reclama** su parte durante la ventana de reclamos, su porción no usada **vuelve al pool de comisiones** y se acumula como **recompensa para el siguiente ciclo**.

Esto genera:

* **Simplicidad** — sin reclamos tardíos ni obligaciones pendientes
* **Equidad** — las recompensas van a participantes activos
* **Componibilidad** — contratos externos pueden implementar estrategias para distribuir recompensas, pero esto es optativo y fuera del alcance del protocolo

Este sistema asegura que los participantes tempranos y comprometidos reciban **ingresos pasivos** proporcionales a su contribución, mientras mantiene la emisión de ASAMI alineada con el uso real del protocolo.

### 5.2 Gobernanza a través del Voto de Comisión

Los tenedores del token ASAMI participan en **un único mecanismo de gobernanza**: definir el porcentaje de comisión del protocolo que determina cuánto del presupuesto de cada campaña se destina al pool de emisión de tokens.

Esta tasa influye directamente en:

* Cuántos tokens ASAMI se emitirán en ciclos futuros
* El valor relativo de ser Divulgador o Administrador de Campaña
* La sustentabilidad del sistema de recompensas del protocolo

En lugar de gestionar campañas, listas blancas o funciones operativas, los tenedores de tokens moldean el **comportamiento a largo plazo de la plataforma** mediante esta única palanca.

> 🗳 **Un Token, Un Voto**
> El voto se pondera según el saldo de ASAMI en cada dirección al momento del snapshot. La tasa final es el promedio ponderado de todos los votos.

#### Ejemplo:

Si la comisión actual del protocolo es 20%, y la comunidad considera que la emisión de tokens es demasiado agresiva, los grandes tenedores pueden votar por una comisión más baja — por ejemplo, 15%. Si la mayoría de los votos apoyan este cambio, el siguiente ciclo ajustará la comisión en consecuencia.

Este diseño asegura que:

* La plataforma siga siendo flexible, pero no caótica
* Los tenedores de tokens expresen preferencias sin microgestión
* Los cambios ocurran de forma incremental, con amortiguación incorporada

La gobernanza está implementada **on-chain** en el contrato inteligente del protocolo y puede auditarse [aquí](https://github.com/constata-eu/asami/blob/38651663a124c714d2e599661f9abf3976e5f628/contract/contracts/AsamiCore.sol#L711).

### 5.3 Modelo de Emisión y Equidad

La emisión del token de Asami está diseñada para ser **proporcional, transparente y justa**. Los tokens solo se emiten cuando alguien **financia una campaña** y otra persona **colabora con ella**, anclando así el valor de ASAMI en una acción real de divulgación.

No hay asignaciones para fundadores, inversores ni insiders. Cada token emitido proviene del uso real de la plataforma.

#### Cómo Funciona

Cada 15 días, el protocolo evalúa:

* Cuántos DOC fueron recolectados en comisiones de colaboraciones registradas
* Si la tasa de emisión debe aumentar o disminuir
* Un máximo de **100,000 ASAMI** por período (meta, no garantía)

Si las campañas bajan, se emiten menos tokens. Si la actividad aumenta, la emisión puede subir — pero **nunca supera el tope**. Esto hace que ASAMI sea una **moneda basada en trabajo**, no en especulación.

#### Distribución de Tokens por Colaboración

Cada colaboración activa una **emisión de ASAMI basada en comisión**. Estos se distribuyen de la siguiente manera:

* 🥜 **40%** para el **Administrador de Campaña** que registró la campaña
* 🥜 **30%** para el **Divulgador** que hizo el repost
* 🥜 **30%** para el **Proyecto** (Anunciante) que financió la campaña

Este modelo refuerza una dinámica saludable:

* Los Administradores de Campaña tienen incentivo para atraer buenas campañas y mantener la calidad
* Los Divulgadores son recompensados por su alcance genuino
* Los Proyectos obtienen equidad a largo plazo al financiar el ecosistema desde temprano

> ❗ **Nota:** Los Proyectos solo reciben ASAMI cuando financian campañas **on-chain en Rootstock** usando DOC. Las campañas financiadas con **Stripe** o futuros medios como **Bitcoin/Lightning** **no** generan ASAMI para los anunciantes — aunque los Divulgadores y Administradores sí reciben su parte.

#### Ejemplo: Ganar 1,000 ASAMI

Si una campaña resulta en la emisión de 1,000 ASAMI:

* 400 van al Administrador de Campaña
* 300 van al Divulgador
* 300 van al Proyecto (si financió con DOC en Rootstock)

No importa tu rol: ganas en proporción a tu contribución. No hay necesidad de hacer staking, bloquear tokens ni especular — solo participar.

El único gran tenedor de tokens hoy es el Administrador de Campaña actual, quien ha registrado todas las campañas hasta la fecha y ha recibido \~40% de los ASAMI emitidos, según las reglas del protocolo.

### 5.4 Reclamar y Mantener Tokens

Los tokens ASAMI **no se envían automáticamente**. Para ser tenedor del token, cada participante debe tomar el **paso deliberado** de reclamarlos.

Este diseño refleja la filosofía de Asami: **la propiedad debe ser intencional** y la participación siempre debe ser voluntaria.

#### Cómo Reclamar tus Tokens

Si ganaste ASAMI por participar en una campaña — ya sea como **Divulgador**, **Administrador de Campaña** o **Proyecto** — puedes reclamarlos en cualquier momento.

Para hacerlo:

* Conecta una **wallet compatible con Rootstock**
* Visita tu perfil público o la página del Token
* Reclama tus tokens acumulados a tu dirección de wallet

También puedes elegir **no reclamarlos de inmediato** — pero los tokens **no reclamados no participan en ingresos**.

#### Mantener Tokens

Una vez reclamados, tus tokens deben **permanecer intactos** durante todo un ciclo de 15 días para calificar para la distribución de ingresos.

Esto significa:

* No transferencias, swaps ni movimientos durante el ciclo
* Los tokens deben estar en una wallet personal (no en pools de liquidez ni contratos)
* Debes mantener al menos **4,000 ASAMI** para calificar

Si mueves tus tokens — incluso por un instante — pierdes tu parte de ese ciclo.

El modelo de reclamar y mantener de Asami ata los incentivos directamente al compromiso. Asegura que solo los participantes activos e intencionales moldeen el futuro del club — y que los primeros en apoyar sean **recompensados de forma modesta y justa** con el tiempo.

---
## 6. Arquitectura Técnica

El protocolo Asami está implementado como un smart contract de código abierto desplegado en la blockchain **Rootstock**. Rootstock es una sidechain de Bitcoin que ofrece compatibilidad total con la EVM, permitiendo a los desarrolladores usar herramientas familiares de Ethereum mientras aprovechan el modelo de seguridad de Bitcoin.

### 6.1 Por qué Rootstock

Rootstock fue elegida porque ofrece:

* **Merged mining con Bitcoin**, lo que incrementa su seguridad.
* **Compatibilidad con la EVM**, permitiendo un despliegue ágil de contratos inteligentes.
* **Alta disponibilidad**, sin historial de caídas y con producción de bloques constante.
* **Entorno nativo de Bitcoin**, que está alineado con los objetivos de descentralización de Asami.

Rootstock utiliza **RBTC** como su moneda de gas nativa y es una red con tarifas bajas y operación estable, ideal como base para Asami.

### 6.2 Por qué Dollar on Chain (DOC)

Los presupuestos de campaña se expresan en **DOC (Dollar on Chain)**, una stablecoin nativa de Rootstock emitida por Money on Chain. DOC es:

* Sobrecolateralizada con Bitcoin
* Ampliamente utilizada en el ecosistema Rootstock
* Indexada 1:1 al dólar estadounidense

Los Divulgadores reciben pagos en DOC. Esto ofrece un mecanismo de recompensa predecible y estable, inmune a la volatilidad del mercado cripto.

> Si un Divulgador republica el mensaje de una campaña y es elegible, recibe un pago en DOC del presupuesto de campaña. El protocolo retiene automáticamente un 10% de este pago como comisión.

### 6.3 Dirección del Smart Contract y Transparencia

El smart contract que impulsa el protocolo es públicamente verificable y puede consultarse en:
[Contrato de Asami en Rootstock Explorer](https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954)

El primer Administrador de Campaña actualmente opera desde:
[Wallet del Administrador de Campaña](https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa)

Todas las interacciones con el protocolo —creación de campañas, registro de colaboraciones y distribución de recompensas— son visibles y trazables en la cadena.

### 6.4 Software del Administrador de Campaña

Los Administradores de Campaña interactúan con el protocolo mediante un software desarrollado en **Rust**, usando el framework web **Rocket**. Esta aplicación gestiona:

* Detección de colaboraciones
* Cálculo de influencia
* Registro de reposts
* Puenteo entre wallets Web2 y Web3
* Solicitudes de retiro sin gas

El código completo es open source y se mantiene en:
[https://github.com/constata-eu/asami](https://github.com/constata-eu/asami)

Esta arquitectura garantiza:

* Modularidad: otros Administradores pueden desplegar su propia versión
* Transparencia: el comportamiento y la lógica de puntuación pueden ser auditados
* Extensibilidad: permite agregar funciones como categorización, referidos y encuestas off-chain

### 6.5 Retiros sin Gas y Onboarding

Para facilitar el onboarding de usuarios nuevos en Rootstock, el Administrador de Campaña ofrece un sistema de **retiros sin gas**. Los Divulgadores:

* Aprueban una deducción de 1 DOC como comisión
* Tienen sus recompensas reclamadas por el Administrador en su nombre
* Reciben su pago en DOC junto con una pequeña cantidad de RBTC para cubrir futuros costos de gas

Este mecanismo facilita la entrada de usuarios no cripto y actúa como un faucet de RBTC, manteniendo la propiedad total sobre las recompensas.

### 6.6 Integración con Pasarelas de Pago

Para mejorar la experiencia de los Anunciantes, los Administradores pueden integrar onramps fiat como **Stripe** para:

* Aceptar pagos con tarjeta o transferencia bancaria
* Convertir automáticamente fondos a DOC
* Financiar campañas directamente desde plataformas Web2
* Aplicar una comisión del 20% para cubrir costos y desincentivar la dependencia off-chain
* Las campañas financiadas con Stripe **no emiten tokens ASAMI**

Esto reduce aún más las barreras de entrada para nuevos anunciantes.

---


## 7. Gobernanza

Asami no está gobernado por una empresa, una DAO ni un contrato social. Está gobernado por lo que es **aplicable mediante software** y lo que es **elegido voluntariamente por los participantes**.

No existen obligaciones entre participantes más allá de lo que está codificado en el smart contract. El protocolo no tiene autoridad integrada, ni comités, ni procesos de toma de decisiones en nombre de otros. Todos los roles son opcionales, y toda conducta está limitada por el código y los incentivos.

Este enfoque minimalista evita la complejidad, política y ambigüedad que suelen surgir en sistemas con estructuras de gobernanza múltiples. En su lugar, adopta la transparencia, la modularidad y la alineación económica. Si alguien quiere que algo funcione diferente, debe:

* Escribir el software que lo haga,
* Convencer a otros de usarlo,
* O bifurcar el protocolo y ofrecer una alternativa.

El modelo de gobernanza de Asami es intencionalmente limitado. Su objetivo es fomentar la **colaboración abierta sin expectativas** y **mantener el poder y la responsabilidad localizados** en el rol que los ejerce.

### 7.1 Participación Voluntaria y Límites de Rol

Cada participante en Asami.club elige su rol libremente y no asume obligaciones más allá de lo que está expresado en el smart contract o en la interfaz pública. Estos roles — **Anunciante**, **Divulgador**, **Administrador de Campaña** y **Titular de Tokens** — son **independientes**, **reemplazables** y **no jerárquicos**.

No hay gobernanza interpersonal, ni equipo central, ni obligación de soporte entre roles. Esta sección detalla lo que cada rol puede esperar y lo que **no** le corresponde exigir.

#### Anunciantes

* Pueden crear campañas para recompensar visibilidad y reclutar divulgadores.
* Pueden decidir si usar listas blancas, listas negras o mantener campañas abiertas a todos los Divulgadores elegibles.
* Son responsables de seleccionar un **Administrador de Campaña** para registrar su campaña en la cadena.
* Al usar el sitio web por defecto (asami.club), están eligiendo al **Administrador de Campaña por defecto** que actualmente opera la plataforma.
* No pueden exigir puntuaciones personalizadas, explicaciones, ni cambios en las decisiones del administrador.
* No tienen derecho a reembolsos, incluso si la campaña recibe pocos o ningún repost.
* Los anunciantes que financian campañas con métodos off-chain (ej. Stripe, LN) podrían no recibir tokens ASAMI.

#### Divulgadores

* Pueden elegir republicar campañas que coincidan con sus intereses y valores.
* Su influencia es evaluada mediante un algoritmo público y recalculada periódicamente.
* **No tienen garantizada** su participación, pago o visibilidad en ninguna campaña.
* Si consideran que fueron mal categorizados o subestimados, pueden presentar un reporte detallado a través del robot de Asami — pero el Administrador de Campaña tiene la última palabra.
* Son libres de dejar de participar o explorar otras plataformas en cualquier momento.

#### Administradores de Campaña

* No forman parte del smart contract. Son **entidades off-chain** que cumplen funciones operativas:

  * Puntuar a los Divulgadores
  * Registrar campañas y colaboraciones
  * Gestionar frontends y brindar soporte
* **Cualquier persona o grupo** con capacidades técnicas puede convertirse en Administrador de Campaña y ejecutar su propio frontend sobre el smart contract.
* Actualmente hay **solo un Administrador de Campaña** operando en asami.club. Su operación está subsidiada, ya que los costos de infraestructura superan lo recaudado en comisiones.
* Los Administradores de Campaña **no están obligados** a explicar decisiones de puntuación ni ofrecer soporte. Tienen plena discreción sobre cómo operar su instancia.

#### Titulares de Tokens

* Votan sobre la tasa de comisión del protocolo, que regula la emisión de nuevos tokens ASAMI.
* No tienen autoridad sobre campañas, puntuaciones ni operaciones de la plataforma.
* Su única influencia es sobre el **balance económico** del protocolo mediante votación.


### 7.2 El Modelo “Tal Como Es” y Sus Consecuencias

No existen acuerdos de nivel de servicio entre las partes. El protocolo existe tal como está, y los participantes aceptan operar dentro de sus límites. Los roles de Anunciante, Divulgador y Administrador de Campaña tienen limitaciones claras:

#### Anunciantes

* No tienen derecho a soporte por parte de los Administradores de Campaña.
* No tienen derecho a reembolsos ni explicaciones si una campaña no recibe reposts.
* Pueden ser rechazados a discreción del Administrador de Campaña.
* No tienen derecho a emisión de tokens cuando financian campañas con Stripe u otros métodos fuera de Rootstock.

#### Divulgadores

* No tienen derecho a participar en campañas específicas.
* No tienen derecho a explicaciones ni ajustes sobre su puntuación.
* No tienen derecho a compensación a menos que hayan participado exitosamente en una campaña según lo definido por el smart contract.
* Pueden abandonar la plataforma en cualquier momento.

#### Administradores de Campaña

* No tienen obligación de reembolsar a anunciantes ni volver a puntuar divulgadores.
* No están obligados a responder quejas.
* Pueden cambiar los criterios y estructura de puntuación según lo consideren.
* Pueden dejar de operar en cualquier momento.

### 7.3 Resolución de Quejas

Asami.club minimiza las obligaciones y mantiene las interacciones transparentes. Como no hay garantías off-chain ni compromisos personales, la mayoría de las quejas no pueden resolverse formalmente. Sin embargo, reportes bien argumentados pueden ayudar a mejorar el sistema con el tiempo.

Si crees que algo está mal — por ejemplo, una puntuación incorrecta, una campaña inadecuada o visibilidad injusta — puedes presentar un reporte para revisión pública. Para ser considerado:

* El reporte debe incluir **evidencia comparativa específica** con enlaces públicos (por ejemplo, perfiles de X, páginas de perfil en Asami, tweets o estadísticas).
* Reclamaciones vagas, emocionales o puramente subjetivas no serán atendidas.
* El enfoque debe ser la equidad relativa: “Esta cuenta recibió recompensa por X, mientras que la mía no, a pesar de Y”.

Los Administradores de Campaña **pueden** considerar estos reportes, pero **no están obligados** a responder, explicar o aplicar cambios. El algoritmo de puntuación se actualiza periódicamente, y los aportes constructivos pueden influir en futuras versiones — pero no se harán ajustes retroactivos.

Si cualquier participante — Anunciante, Divulgador o Titular de Tokens — no está conforme con cómo opera un Administrador de Campaña:

* Puede dejar de utilizar sus servicios.
* Puede apoyar o financiar la creación de un **Administrador de Campaña alternativo** que aplique otras reglas, haga una curaduría diferente o utilice otro modelo de puntuación.

El sistema **se autorregula mediante la elección y la transparencia**, no mediante la coerción ni la apelación. El Administrador de Campaña actual **no tiene acceso especial ni privilegios** en el smart contract. Cualquiera con conocimientos técnicos puede registrar campañas y colaboraciones, o construir su propio frontend para servir a otros usuarios y preferencias.


### 7.4 Votación de la Tasa de Comisión

Asami no tiene gobernanza de propósito general. Los Titulares de Tokens no deciden quién participa, cómo se asignan las puntuaciones ni qué Divulgadores o Proyectos son admitidos.

Sin embargo, sí votan sobre **un parámetro clave**: la **tasa de comisión del protocolo**. Este es el porcentaje que se cobra a los anunciantes cuando financian una campaña, y determina cuántos tokens ASAMI se emiten como resultado de esas comisiones. Una tasa más alta implica mayor emisión de tokens y una mayor reserva a distribuir.

#### Cómo Funciona la Votación

* Cada 15 días comienza un nuevo “ciclo de contrato”.
* Durante cada ciclo, los Titulares de Tokens pueden indicar su tasa preferida votando directamente on-chain.
* El sistema calcula un **promedio ponderado** de todos los votos según el saldo de tokens de cada votante.
* Al final del ciclo, se fija la tasa seleccionada y se aplica al período siguiente.

Ejemplo:

* Si la mayoría vota por una tasa del 20%, pero una ballena con el 60% del suministro vota por 10%, la tasa resultante se acercará al 10%, pero seguirá reflejando el promedio ponderado.

Los titulares pueden ver la tasa actual, los votos anteriores y los ciclos futuros en: [https://asami.club/#/Token](https://asami.club/#/Token)

Este es el **único mecanismo de gobernanza en Asami**, y está diseñado para:

* Permitir a los titulares de tokens influir indirectamente en la emisión y en los incentivos.
* Evitar propuestas abiertas, interpretaciones subjetivas o presiones sociales.
* Mantener una gobernanza mínima, mecánica y predecible.

---

## 8. Comunicación, Transparencia y Seguridad

Asami fue diseñado para ser abierto por defecto — no solo a nivel del protocolo on-chain, sino también en cómo interactúa con Divulgadores, Proyectos y Administradores de Campaña. Cada aspecto del sistema está pensado para fomentar la visibilidad, la rendición de cuentas y el debate público.

La transparencia genera confianza. Pero la apertura también puede habilitar abuso, confusión o situaciones límite. Por eso Asami equilibra su transparencia con una capa modesta pero cuidadosa de medidas de seguridad, filtros de calidad y pautas de soporte — todas pensadas para proteger el ecosistema sin socavar su naturaleza permissionless.

En esta sección, explicamos cómo se comunica Asami, cómo se puede verificar su información y cómo se previene la manipulación o el mal uso — manteniendo siempre el espíritu del club: colaboración pública y voluntaria.

### 8.1 Diseño Orientado a lo Público

Asami.club comunica abiertamente por diseño. En lugar de depender de chats privados o foros cerrados, el club mantiene su presencia y la mayoría de sus discusiones en espacios públicos — especialmente en X (antes Twitter). Esto promueve la responsabilidad, reduce la ambigüedad y ayuda a nuevos usuarios a entender el sistema observándolo en acción.

Las solicitudes de soporte se gestionan por canales definidos, y solo para problemas técnicos específicos. Los grupos públicos de soporte en Telegram existen para casos como:

* El sitio web no funciona como se espera.
* No puedes vincular tu cuenta de X, correo electrónico o wallet.
* Registraste una campaña o hiciste repost, pero no aparece o no fue pagado después de un tiempo razonable.
* Intentaste crear una campaña que nunca se mostró.

Todas las demás consultas deben hacerse primero al [Robot de Asami](https://robot.asami.club), que puede ayudar a los divulgadores a explorar datos públicos, entender su puntuación de influencia y preparar un reporte adecuado. Los comentarios generales o feedback deben hacerse de forma pública, mencionando la cuenta oficial del club (por ejemplo, `@asami_club_es`) en X.

Los canales de soporte no son centros de ayuda generales, ni espacios para debatir puntuaciones de influencia o solicitar acceso a campañas. Los moderadores pueden ignorar o eliminar mensajes que no respeten las reglas fijadas en los mensajes anclados.

Este enfoque público es intencional. Refleja los valores de Web3 — donde los participantes toman iniciativa, exploran datos abiertos y se relacionan con contratos y código, no con autoridades centralizadas.

### 8.2 Transparencia por Diseño

Asami.club se basa en el principio de verificabilidad. Si bien no todos los datos individuales pueden hacerse públicos debido a restricciones de plataformas —como los términos de la API de X—, el sistema está diseñado para asegurar que todas las decisiones estructurales, la lógica central y los resultados derivados sean transparentes y estén abiertos al escrutinio.

Componentes clave disponibles para inspección pública:

* **Metadatos de campaña**: quién creó la campaña, qué Divulgadores participaron, qué recompensas se prometieron y pagaron, y cuántas impresiones se lograron.
* **Perfiles de Divulgadores**: accesibles públicamente en Asami, muestran su puntuación de influencia, historial de campañas y ganancias.
* **Datos de tokens e ingresos**: total de tokens ASAMI emitidos, saldos del fondo de comisiones, eventos de distribución y estadísticas históricas en [asami.club/#/Token](https://asami.club/#/Token).
* **Registros de colaboraciones**: incluyendo la campaña, el repost, el pago y la fecha — todo verificable y almacenado on-chain.

El **algoritmo de puntuación de influencia es completamente open source y versionado**, para que cualquiera pueda entender cómo se calculan los puntajes y cómo evoluciona el sistema. Aunque los datos en bruto usados en el scoring —como impresiones y métricas de engagement de X— no pueden compartirse por los términos de la plataforma, la metodología y los resultados son auditables dentro de esos límites.

El rendimiento de las campañas, la emisión de tokens y las contribuciones de los participantes son verificables a través de interfaces públicas o inspección on-chain. No hay backend privado ni privilegiado; todos los usuarios, proyectos y terceros interactúan con los mismos datos transparentes.

Este diseño promueve la responsabilidad sin depender de la confianza en actores específicos.

### 8.3 Prevención de Abuso y Mal Uso

Asami incluye múltiples capas de protección para reducir la manipulación, el spam y el comportamiento abusivo — especialmente en torno a la puntuación de influencia, participación en campañas y distribución de recompensas.

El diseño de Asami no elimina completamente el abuso, pero desalienta la participación de baja calidad y recompensa la sinceridad. Los administradores de campaña y proyectos tienen la capacidad de mantener la calidad tanto con código como con criterio.

#### Salvaguardas Algorítmicas

* **Puntuación base**: Los Divulgadores se filtran con un algoritmo de influencia que descarta cuentas inactivas o no genuinas. Solo quienes superan el umbral mínimo pueden ver y participar en campañas.
* **Ratios de engagement**: El algoritmo penaliza cuentas con muchas impresiones pero poco engagement real, desincentivando visibilidad artificial o métricas impulsadas por bots.
* **Señales verificadas**: El algoritmo recompensa seguidores verificados e interacciones con publicaciones de alta visibilidad. Esto dificulta inflar la puntuación con cuentas falsas o métricas superficiales.
* **Entradas manuales**: El Administrador de Campaña puede complementar la puntuación con indicadores de influencia offline (por ejemplo, organización de eventos, liderazgo comunitario), lo que permite una evaluación más matizada y contextual.

#### Restricciones de Plataforma

* **Filtrado por puntuación obligatorio**: Solo cuentas por encima de cierto umbral —definido por el Administrador de Campaña— pueden participar en campañas. Esta restricción se aplica de forma global y no es configurable por los anunciantes.
* **Presupuesto y duración son unilaterales**: El presupuesto y la duración de una campaña pueden ampliarse después de su creación, pero nunca reducirse. Las campañas no pueden finalizarse anticipadamente, ni retirarse fondos una vez comprometidos.
* **Detalles off-chain**: Otros parámetros de campaña (como contenido del mensaje, idioma o categoría) no se almacenan on-chain. Son gestionados por el Administrador de Campaña y pueden modificarse —con criterio— por motivos de claridad, calidad o moderación.

#### Controles Sociales y Estructurales

* **Criterio del anunciante**: Los Proyectos pueden incluir en listas negras o blancas a Divulgadores después de su participación. Con el tiempo, esto les permite refinar quién puede interactuar con sus campañas futuras.
* **Incentivos públicos**: Todas las recompensas y reposts son públicos, promoviendo responsabilidad moral. Cuando un Divulgador acepta un pago, reconoce públicamente su rol en amplificar el mensaje — a diferencia del marketing de influencers tradicional, donde la compensación suele ser oculta.

### 8.4 Responsabilidad y Transparencia

Asami promueve la responsabilidad y el comportamiento ético haciendo visibles los reposts y las recompensas. Cuando un Divulgador republica un post de campaña, su perfil público muestra claramente:

* El contenido que eligió amplificar.
* El hecho de que recibió (o recibirá) una recompensa por hacerlo.
* El monto exacto y tipo de recompensa (DOC y ASAMI).

Este nivel de transparencia es clave para generar confianza. A diferencia de las promociones encubiertas o patrocinios no revelados, la participación en campañas de Asami es **transparente por diseño**. Tanto Proyectos como Divulgadores se benefician de un sistema donde la visibilidad y la compensación están abiertamente vinculadas.

Este diseño tiene un efecto adicional: al aceptar una recompensa pública, el Divulgador **asume implícitamente cierta responsabilidad** por lo que ayudó a promocionar. La recompensa no es solo un agradecimiento — es una señal de que el Divulgador tomó una decisión consciente de asociarse con un proyecto. Esto diferencia a Asami de otras plataformas que permiten promociones anónimas o sin consecuencias.

Se anima a los Divulgadores a promover únicamente lo que realmente apoyan. Aunque la recompensa sea modesta, el acto de aceptarla transmite confianza e intención — no una aprobación ciega — una distinción que fortalece la credibilidad de todo el ecosistema.

---

**9. Market-fit y panorama competitivo**

Los proyectos Web3 cuentan con una amplia variedad de herramientas para llegar a nuevos usuarios, aumentar su visibilidad e incentivar la participación. Desde airdrops hasta quests o campañas sociales virales, el ecosistema de crecimiento es rico y variado — y con razón. Cada objetivo requiere tácticas distintas.

Asami no busca reemplazar estas herramientas. No intenta incorporar usuarios mediante dinámicas de juego, ni recompensar interacciones puntuales. En cambio, se enfoca en un aspecto específico pero frecuentemente ignorado de la adopción Web3: la **divulgación**. Personas que aprenden, creen y comparten — no por dinero, sino porque quieren que otros comprendan lo que han descubierto. Son quienes explican, popularizan y recomiendan. Su rol en el descubrimiento de proyectos es esencial, y Asami les ofrece una forma modesta y transparente de ser apreciados y reconocidos por ello.

Asami opera en un espacio donde se superponen múltiples estrategias de crecimiento, influencia y marketing. Si bien su enfoque está en reconocer y recompensar la divulgación Web3 mediante incentivos transparentes y modestos, puede cruzarse con herramientas orientadas a la publicidad, onboarding, monetización o gestión de influencers. Por ejemplo, un proyecto puede usar Galxe o TaskOn para ejecutar campañas incentivadas que premian a los usuarios por probar un producto. Al mismo tiempo, puede usar Asami para llegar a verdaderos divulgadores que **comparten la campaña** con su audiencia, añadiendo credibilidad y alcance. Este enfoque por capas puede amplificar la efectividad de estrategias tradicionales, anclándolas en apoyo comunitario auténtico.

El valor único de Asami está en convertir la divulgación en una interacción visible y rastreable — creando un registro público de quién apoya qué, y cuándo, manteniendo los incentivos modestos y vinculados a la reputación.

Esta sección describe las categorías de plataformas con las que Asami suele ser comparado y aclara en qué medida representan competencia, complemento o simplemente cumplen otro propósito. En muchos casos, Asami puede usarse junto con estos otros métodos — especialmente cuando una campaña requiere tanto visibilidad como credibilidad entre audiencias familiarizadas con Web3.

### 9.1 El marketing Web3 hoy

El marketing en el espacio Web3 enfrenta desafíos únicos. Los canales de publicidad tradicionales a menudo imponen restricciones al contenido relacionado con cripto, y algunas plataformas directamente prohíben o limitan la visibilidad de promociones blockchain. Incluso cuando están permitidos, los anuncios de proyectos descentralizados pueden parecer poco confiables o irrelevantes para audiencias que no están familiarizadas con los conceptos de Web3.

Para sortear estos obstáculos, muchos proyectos dependen de **plataformas basadas en incentivos** que recompensan a los usuarios por acciones específicas: instalar una wallet, probar una dApp, unirse a una comunidad o completar una tarea. Estas estrategias pueden generar grandes volúmenes de interacciones y usuarios en poco tiempo. Sin embargo, rara vez resultan en **retención a largo plazo** o **interés auténtico**. Los participantes suelen completar las tareas para obtener una recompensa y marcharse poco después, inflando métricas sin construir comunidades duraderas.

Mientras tanto, gran parte del crecimiento real que han experimentado los proyectos Web3 fundamentales —desde Bitcoin hasta infraestructura open-source— no ha sido producto de la publicidad, sino de los primeros **divulgadores**: individuos que se tomaron el tiempo de explicar, recomendar y promover lo que creían importante. Esas personas moldearon el discurso, respondieron preguntas y ayudaron a otros a dar sus primeros pasos en Web3. Sin embargo, históricamente, ese trabajo de convicción y comunicación ha pasado mayormente desapercibido.

Web3 necesita ambos tipos de esfuerzo: incorporación escalable de usuarios y construcción de comunidad creíble a largo plazo. Asami se enfoca en lo segundo.

### 9.2 Acciones incentivadas vs. divulgación genuina

Plataformas como **Galxe**, **TaskOn** y **Zealy** recompensan a los usuarios por completar acciones predefinidas — como registrarse, usar una función o unirse a una comunidad. Estas herramientas son útiles para el onboarding y pueden aumentar rápidamente el engagement, pero la actividad resultante suele ser transaccional y de corta duración.

**Asami ofrece un enfoque complementario**: apoya a quienes ya están hablando bien de un proyecto — no porque se les pidió, sino porque creen en su propuesta. En lugar de incentivar un comportamiento específico, Asami **reconoce públicamente y recompensa modestamente a los simpatizantes reales** que amplifican campañas mediante reposts.

Esta diferencia importa. Las acciones incentivadas son efectivas para generar participación inicial; Asami se enfoca en **reconocer la convicción**. Ambos enfoques pueden coexistir: por ejemplo, un proyecto puede lanzar una quest en Galxe y usar Asami para difundirla a través de voces confiables — dando a la campaña tanto alcance como credibilidad.

### 9.3 Plataformas descentralizadas de marketing con influencers

Proyectos como **Chirpley** y el extinto **D-Cent** buscaron descentralizar el marketing con influencers conectando directamente marcas con usuarios de redes sociales. Chirpley, en particular, permite a los anunciantes elegir entre un mercado de microinfluencers categorizados por intereses como belleza, gaming o viajes.

**Asami adopta un enfoque diferente**, centrado en la **descubierta orgánica**. Los anunciantes no seleccionan a los participantes por adelantado. En cambio, las campañas se muestran solo a Divulgadores preseleccionados mediante puntuación de influencia, y cualquiera que haga repost se vuelve visible para el anunciante. Esto permite a los proyectos **identificar a los simpatizantes alineados con el tiempo** y crear listas blancas selectivas para campañas futuras. El modelo equilibra **exploración y explotación** — descubriendo nuevas voces mientras se cultiva un grupo confiable.

Chirpley prioriza la segmentación detallada y el soporte a múltiples industrias. Asami se enfoca exclusivamente en Web3, aplicando su propio algoritmo de influencia para mantener un conjunto filtrado de Divulgadores de calidad. La segmentación en Asami es manual, realizada por el Campaign Manager según el comportamiento público y el uso del lenguaje — no por intereses autodeclarados.

Mientras que Chirpley ejecuta campañas basadas en tareas definidas, Asami prioriza la **visibilidad, la divulgación genuina y el crecimiento comunitario a largo plazo**. En una estrategia combinada, un proyecto podría usar Chirpley para activar influencers contratados y Asami para **reconocer y recompensar a quienes realmente creen en su misión**.

Por último, cabe destacar que el token **ASAMI no se utiliza como método de pago**, sino como una recompensa tipo equity para quienes ayudan a hacer crecer el club. Tiene un suministro fijo de 21 millones y solo se emite cuando hay actividad real en el protocolo. Los pagos a Divulgadores se realizan en **DOC**, una stablecoin, lo que aporta estabilidad para los proyectos y claridad para los participantes.

### 9.4 Plataformas sociales con funciones de monetización

Redes como **Facebook**, **Instagram**, **X (antes Twitter)** y **TikTok** ofrecen herramientas de monetización integradas, que permiten a los creadores generar ingresos mediante contenido patrocinado, reparto de ingresos publicitarios o apoyo directo de sus seguidores. Estas plataformas dominan la atención global y ofrecen vías escalables para quienes quieren vivir de su audiencia.

**Asami tiene otro objetivo.** No busca convertirse en la fuente principal de ingresos para creadores de contenido. En cambio, apunta a aquellas personas que actúan como **divulgadores y educadores**: individuos que comparten contenido significativo sobre proyectos Web3 por interés genuino, convicción personal o deseo de educar a otros.

Estos Divulgadores reciben pagos modestos en **DOC** al repostear campañas que resuenan con sus valores. El objetivo no es generar un sueldo, sino ofrecer **reconocimiento y una participación simbólica** en las comunidades que ayudan a crecer.

Los usuarios de Asami pueden seguir utilizando plataformas tradicionales para monetizar su trabajo (como YouTube, TikTok o Twitch). En este sentido, **Asami complementa esos canales**, agregando una capa transparente y ligera de incentivos para contenido que ya estaban dispuestos a compartir por convicción.


### 9.5 Redes sociales basadas en blockchain

Existen plataformas que combinan infraestructura blockchain con redes sociales para ofrecer monetización tokenizada, resistencia a la censura y propiedad comunitaria. Veamos cómo se comparan con Asami:

* **Hive** (anteriormente una bifurcación de Steem) es una red social descentralizada donde los usuarios ganan tokens HIVE o HBD publicando contenido y votando, gracias al mecanismo de “Proof-of-Brain”. Su foco está en la publicación on-chain, no en recompensar la amplificación de contenido ajeno. **Asami**, en cambio, opera sobre **redes sociales ya existentes** (como X), y recompensa con **pagos en DOC y tokens ASAMI** a los usuarios que amplifican campañas Web3 genuinamente. No requiere migrarse ni publicar contenido original en una blockchain.

* **DeSo** (antes BitClout) permite especular sobre la reputación de personas mediante “creator coins”. Su enfoque es **especulativo**, mientras que **Asami mide y recompensa el impacto social práctico** mediante reposts y un algoritmo transparente de influencia.

* **Lens Protocol**, **Farcaster** y similares ofrecen identidad descentralizada y potencial futuro para el contenido social. Sin embargo, aún carecen de escala y de herramientas de campañas estructuradas como las que Asami ya aplica en plataformas mainstream. **Asami se enfoca en generar impacto medible en redes sociales ampliamente utilizadas**, sin descartar expandirse a redes descentralizadas cuando estas alcancen mayor adopción.

**Conclusión**: Asami no intenta reinventar la red social, como hacen Hive o DeSo. En su lugar, se integra con canales existentes y recompensa acciones verificables que impulsan campañas Web3 — con transparencia, trazabilidad y sin fricción.

### 9.6 Plataformas que venden reposts a anunciantes

Servicios como **WeAre8**, **SocialBoost**, **InstaFollowers** y similares permiten a los anunciantes **comprar reposts, likes y engagement**, generalmente priorizando volumen sobre autenticidad.

* **WeAre8** reparte ingresos publicitarios entre usuarios que ven o interactúan con contenido patrocinado. Aunque se presenta como una alternativa ética, no filtra por **interés genuino** ni por experiencia en el tema.
* Otras plataformas como **SocialBoost** o **InstaFollowers** venden paquetes de reposts o seguidores sin criterios de calidad, sin transparencia y sin verificar la relevancia del público.

**Asami es lo opuesto a estos enfoques**:

* Las interacciones **no se compran por paquete**. Solo los Divulgadores que han pasado el filtro de influencia y reputación pueden participar.
* Todos los reposts y recompensas están **registrados públicamente en blockchain**, lo que **dificulta el fraude y facilita la auditoría**.
* Cada Divulgador es **filtrado por score**, con base en visibilidad real y autoridad social — no simplemente dispuesto a cobrar por postear.

Para los anunciantes, **Asami ofrece calidad en lugar de cantidad**: amplificaciones modestas, pero auténticas, desde perfiles genuinamente interesados en el ecosistema Web3. Además, puede actuar como **una capa previa** para identificar voceros confiables antes de lanzar una campaña masiva, o como **una capa complementaria** que añade credibilidad a una estrategia de mayor escala.


### 9.7 Proyectos que miden la influencia en redes sociales

Plataformas como **FeiXiaoHao** y proyectos de análisis como **TweetBoost** buscan cuantificar la influencia, el sentimiento y la viralidad de contenido en redes sociales. Por ejemplo:

* **FeiXiaoHao** monitorea menciones, tasas de engagement y sentimiento de palabras clave en el ecosistema cripto, ayudando a detectar tendencias y cambios en el ánimo del mercado.
* **TweetBoost** investiga cómo métricas de Twitter —seguidores, retuits, likes— afectan el valor percibido de NFTs, y ha demostrado que incorporar estos datos mejora predicciones de mercado en hasta un 6%.

Estos proyectos comparten con Asami el interés por la **medición de influencia**, pero su objetivo es distinto. FeiXiaoHao y TweetBoost se centran en ofrecer **insights analíticos generales** para traders, investigadores o equipos de marketing. Su enfoque es pasivo: observar, analizar y reportar.

**Asami, en cambio, convierte esa medición en acción.** Su algoritmo evalúa Divulgadores en X, con métricas específicas como impresiones, reposts y ratios de engagement. El resultado no es un informe, sino la **activación directa de campañas**, con pagos transparentes en DOC y distribución de tokens ASAMI. Mientras que un proyecto puede usar herramientas como TweetBoost para planear, Asami le permite **ejecutar** —identificando y recompensando a quienes realmente amplifican su mensaje.

### 9.8 Redes de anuncios centralizadas en redes sociales

Canales tradicionales como **Google Ads**, **Meta Ads (Facebook/Instagram)** o los **posts promocionados de X** ofrecen alcance masivo y herramientas avanzadas de segmentación. Pero para proyectos Web3 presentan obstáculos concretos:

* **Limitaciones regulatorias**: Muchas plataformas restringen o prohíben la publicidad cripto, exigiendo licencias, verificaciones por país o cumplimiento normativo difícil de cumplir.
* **Baneos globales**: TikTok, por ejemplo, ha prohibido reiteradamente los anuncios de criptomonedas por riesgos de protección al consumidor.
* **Costos elevados y poca credibilidad**: La saturación de contenido pago ha disminuido su efectividad. Promocionar un post en X puede costar más de \$100 USD por 25k–50k impresiones, sin garantía de engagement ni confianza del público.

**Asami es una alternativa complementaria**, no un reemplazo. En lugar de pagar por impresiones genéricas, permite que proyectos Web3 consigan **amplificación genuina** por parte de Divulgadores verificados —usuarios que ya tienen influencia y deciden compartir el contenido por afinidad, no por consigna.

Esto genera:

* **Mejor engagement**, al venir de voces creíbles.
* **Mayor confianza**, al ser visible quién apoya qué.
* **Transparencia en pagos y resultados**, al estar todo registrado on-chain.

Para campañas en Web3, Asami actúa como una **capa de credibilidad** que los ads tradicionales no pueden ofrecer —especialmente en contextos donde la confianza y la transparencia son clave.


## 10. Direcciones futuras

Asami sigue siendo un proyecto en evolución, guiado por las necesidades cambiantes de los proyectos Web3 y la comunidad creciente de divulgadores que los apoyan. Esta sección describe áreas de desarrollo planificadas o en curso, orientadas a mejorar la equidad, la flexibilidad y el alcance, sin perder de vista el propósito central de Asami: recompensar la defensa genuina de manera modesta y transparente.

### 10.1 Mejoras en la puntuación de influencia

Las futuras versiones del algoritmo de puntuación refinarán cómo se mide la autoridad y la audiencia, especialmente **por acción individual**. Los tweets citados, las menciones y el engagement específico de cada publicación podrían ponderarse de forma independiente. Esto permitirá recompensar a los divulgadores no solo por su presencia general, sino también por el rendimiento de cada repost, respuesta o mención. También se están considerando técnicas para detectar **influencia inversa** o intentos de manipulación.

### 10.2 Nuevos tipos de actividad elegibles

Actualmente, Asami recompensa reposts y tweets citados por igual, pero otros formatos —como **respuestas** o **menciones**— podrían volverse elegibles en el futuro. Estos se evaluarán según su alcance, intención y verificabilidad, y podrían tener puntuaciones o criterios de acceso distintos, según los requerimientos de cada campaña.

### 10.3 Soporte para otras plataformas

Aunque el enfoque actual está en **X (Twitter)**, Asami planea integrar otras plataformas como **Nostr**, **LinkedIn** e **Instagram**. También se consideran objetivos como **Farcaster**, **Threads** y **Telegram**, siempre que permitan visibilidad pública de publicaciones y recopilación fiable de datos. Asami priorizará plataformas alineadas con los ideales Web3 y el acceso abierto a la información.

### 10.4 Soporte multicripto (BTC y Lightning)

Se encuentra en desarrollo el financiamiento de campañas mediante **BTC** y la **Lightning Network**. Estos métodos de pago se convertirán automáticamente a **DOC** para mantener consistencia en la lógica de recompensas y el cálculo de comisiones. Tendrán comisiones de procesamiento más bajas que Stripe, pero las campañas financiadas de esta forma **no generarán tokens ASAMI** para los participantes.

### 10.5 Cultura de divulgación y alcance

Más allá de las mejoras técnicas, Asami tiene el compromiso de fomentar una cultura de la divulgación. Esto incluye iniciativas educativas para ayudar a las personas a ser mejores divulgadores, apoyo a debates públicos sobre puntuación y equidad, y contenido explicativo sobre cómo participar de forma efectiva. Asami también invertirá en difusión y promoción para aumentar el conocimiento del proyecto y atraer más miembros al club — especialmente aquellos que ya están amplificando ideas Web3 de forma orgánica.

---

## 11. Consideraciones legales

Asami es un protocolo y una plataforma que facilita la colaboración voluntaria entre anunciantes y usuarios de redes sociales (los llamados **divulgadores**) en un entorno transparente y descentralizado. No es una empresa, fundación ni entidad legal. Aunque el software y la infraestructura puedan ser mantenidos por personas u organizaciones identificables, el protocolo en sí opera mediante contratos inteligentes inmutables y gobernanza abierta a través de la emisión de tokens.

### 11.1 No es una oferta de inversión

Los tokens **ASAMI** no se ofrecen como una inversión. Representan participación en el protocolo Asami.club y se emiten a los contribuyentes que apoyan el ecosistema ya sea promoviendo contenido (divulgadores), gestionando campañas (administradores de campaña) o aportando fondos (anunciantes). El token tiene una oferta fija y se distribuye según reglas predeterminadas codificadas en un contrato inteligente. No hay ninguna promesa de ganancia ni entidad que garantice su apreciación en valor.

### 11.2 Sin garantías ni compromisos

Asami no proporciona garantías de servicio a ningún participante. El comportamiento del protocolo está definido completamente por su código, y todas las interacciones son bajo la discreción y riesgo de los participantes. No existe obligación legal para que los administradores de campaña puntúen de forma justa, respondan a reclamos o emitan reembolsos. Las quejas deben resolverse socialmente, por reputación, o migrando a una infraestructura alternativa construida sobre el mismo protocolo.

### 11.3 Cumplimiento normativo y leyes locales

Los usuarios son responsables de cumplir con las leyes y normativas locales que les correspondan, incluyendo pero no limitándose a:

* Divulgar promociones pagadas cuando sea legalmente requerido.
* Pagar impuestos sobre ingresos obtenidos a través de campañas.
* Respetar los términos de uso de las plataformas al interactuar con servicios de terceros como **X (Twitter)**.

Asami no puede ni intenta hacer cumplir estos requisitos, pero **fomenta una participación responsable y conforme a la ley**.

### 11.4 Privacidad y datos

Asami no recolecta datos personales más allá de lo que se publica en la cadena. Sin embargo, los administradores de campaña y otros participantes pueden recolectar o analizar datos públicos de redes sociales para evaluar la influencia. El uso de estos datos debe respetar los términos y limitaciones de las plataformas de origen (por ejemplo, los términos de API de X). Los conjuntos de datos internos utilizados para el puntaje pueden no ser compartibles, aunque el algoritmo sí sea público.

Los administradores de campaña pueden almacenar datos personales o seudónimos para mejorar sus servicios o mantener historial de puntuaciones. Estos datos pueden ser **eliminados a pedido**. Si tienes preguntas o preocupaciones sobre tu información, puedes escribir a **[dpo@asami.club](mailto:dpo@asami.club)** para recibir asistencia.


Claro, aquí tienes la traducción al español neutro, manteniendo el formato Markdown intacto:

---

## 12. Notas Finales

Asami.club es un experimento en evolución constante para construir una forma de divulgación en redes sociales más transparente, modesta y orientada a la comunidad. No pretende resolver todos los problemas del marketing o de los influencers, ni reemplazar la enorme infraestructura de las redes de anuncios centralizadas. En cambio, ofrece una alternativa: una que comienza de forma modesta, recompensa a personas reales y fomenta la participación genuina en el crecimiento de proyectos Web3.

El protocolo es público. Los contratos inteligentes son sin permisos. El club está abierto para todos.

Si eres divulgador, prueba participar en una campaña.

Si eres anunciante, prueba financiar una.

Si eres desarrollador, intenta crear tu propia interfaz al protocolo.

El sistema se convertirá en lo que colectivamente hagamos de él.

---

## Apéndice A: Direcciones de Contrato y Enlaces { .no-cols }

**Contrato inteligente de Asami** (Rootstock):
[https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954](https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954)

**Dirección del Administrador de Campañas actual**:
[https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa](https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa)

**Información del stablecoin DOC (Money on Chain)**:
[https://moneyonchain.com/](https://moneyonchain.com/)

**Software open source del Administrador de Campañas**:
[https://github.com/constata-eu/asami](https://github.com/constata-eu/asami)

---

## Apéndice B: Contacto y Soporte { .no-cols }

Se invita a los miembros a hacer preguntas o pedir ayuda a través de canales públicos. Buscamos mantener toda la comunicación transparente y orientada a la comunidad.

**X (Twitter):**

* Inglés: [@asami\_club](https://twitter.com/asami_club)
* Español: [@asami\_club\_es](https://twitter.com/asami_club_es)

**Telegram:**

* Grupo en inglés: `@asami_club`
* Grupo en español: `@asami_club_es`

Ten en cuenta que todas las solicitudes de revisión y puntuación deben seguir el proceso basado en evidencia descrito en este whitepaper, el cual también contiene un apéndice de preguntas frecuentes.

---

## Apéndice C: Preguntas Frecuentes y Problemas Conocidos { .no-cols }

**P: ¿Por qué no me aparecen campañas?**

* Puede que tu puntuación de influencia sea baja o que no tengas las categorías requeridas. Puedes solicitar una revisión.

**P: ¿Por qué mi puntuación parece demasiado baja?**

* Lee la sección de Medición de Influencia y sigue el proceso de resolución de disputas descrito en la Sección 3.2.

**P: Antes ganaba más que ahora. ¿Por qué?**

* Puede deberse a presupuestos más bajos por parte de anunciantes, cambios en tu puntuación o categoría, menor actividad en la plataforma o mayor competencia.

**P: Fui excluido o bloqueado por el Administrador de Campañas. ¿Aún puedo participar?**

* Sí. Asami es sin permisos. Puedes trabajar con otro administrador de campañas o usar tus tokens ASAMI normalmente.

**Problema conocido:** Algunas colaboraciones pueden registrarse después de que una campaña se quede sin fondos. La plataforma las mostrará como "fallidas", pero podrían ser compensadas más adelante si se agregan fondos.

