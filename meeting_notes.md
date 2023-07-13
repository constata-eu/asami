# Notas reunión 12 de julio.

* Cambio un poco la idea / premisa:
  * Motivado por el costo de validar las firmas de mensajes de nostr en rsk nos pusimos a reflexionar sobre la realidad de costo/beneficio y eficiencia de los "micro-contratos de micro-influencing".
  - En nuestra propuesta original: "A los influencers los inventa twitter para bajar el costo de marketing". no es tan así, en cualquier caso los influencers en nostr van a existir más legítimos, sin intervención de una plataforma. Siguen siendo un objetivo deseable para transmitir mensajes eficientemente.
  - Adherimos al modelo tradicional de tener influencers vs marketing horizontal.
    - Es mas eficiente un mensaje transmitido por alguien que tiene audiencia más grande vs. muchos mensajes entre personas sin audiencia.
    - Las iniciativas como el zapvertising (envio el ad asociado a un pago) no necesariamente están bien enfocadas, cuestan más, y si todos hacen ese tipo de advertising, se puede volver spam, el usuario toma el dinero y no lee la nota. Tampoco resuelve la genuidad del mensaje transmitido.
    - El contenido de los ads es mejor dejarlo en manos del influencer, que de la plataforma. El mensaje llega mejor en el contexto que le da la persona que ya tiene la audiencia.
    - Los nodos mas centrales de cada cluster son mas eficientes de usar para propagar un mensaje. Se puede meter menos ruido y gastar menos también.
  - RSK es solo un garante / escrow.
    No es una necesidad operativa constate. El influencer podría interactuar siempre con el robot, solo recurre a RSK si no le pagan.
  
    Puede ser para las primeras transacciones solamente, pero ayuda a entablar la relación entre las partes.

  - Nuestro trabajo no es "cotizar" cada cuenta de nostr (trabajo gigante), más bien es identificar clusters y nodos centrales de cada uno para valorar si les proponemos hacerse influencers. (como es descentralizado en nostr es difícil).


* Validación de Product market fit:
  - Estamos buscando anunciantes.
    - Formas de buscarlos:
      - Contacto directo (friends & fools).
        - Sin novedades. (salvo Ongun).
      - Proyectos conocidos relevantes del ecosistema RSK.
        (sovryn, tropykus, ...)
      - Campañas existentes:
        Las buscamos por "tags" en nostr.
        - Tenemos la lista pero no hicimos nada todavía.
    - Oferta que hacemos:
      - Les hacemos una campaña de 200USD, les presentamos resultados, nos pagan sin les gusta.


* Requerimientos técnicos / tecnológicos:
  - Robot de indexación de nostr:
    1) Obtener los datos: Un cliente de nostr que busca todos los mensajes agresivamente y los mete en una DB.
    2) Identificar personas genuinas de robots (establecer los criterios, y aplicarlos)
    3) Dibujar el mapa de clusters, identificar nodos centrales.

  - Generador de informes/reporte para el anunciante: 
    - (Va a estar basado en la indexación)
    - Llega por mensaje de nostr.
    - Resumen modo texto para el mensaje.
    - Con cierta frecuencia de actualización.
    - Versión HTML responsive SVG blabla enviada a un CDN (centralizado o RIF storage).

  - Visualizaciones web: De la red, de los clusters, estadística general.
    - Actualizadas periódicamente y subidas a una CDN.

  - El smart contract que gestiona la campaña y los cobros.

  - El robot de chat nostr que hace de "UI".
    - Permite cobrar a los influencers y observa la participación en la campaña.
    - Los contacta con propuestas de ads.

  * Un CRM de influencers y advertisers (usaríamos uno que ya existe, eventualmente puede aparecer una empresa de medios.)

# Notas reunión 10 de julio.

Preguntas: Qué estamos ofreciendo? que cosas asumimos? y cuál es la forma más rápida y barata de comprobar si estamos asumiendo bien?

* A Influencers: Poder monetizar su cuenta.
  - Garantizandole que va a cobrar por su publicación.

* A Auspiciantes: Poder difundir su marca o mensajes mediante influencers.
  - Garantizando que el presupuesto se usa eficientemente. (on-topic, influencers genuinos y no bots, etc).

Outreach Auspiciantes:
- Buscar intentos de campañas en Nostr, ver si les interesa pagarle a los que difunden.
- Ofrecer al bitcoiner usuario de nostr como audiencia a: Sovryn, MoC, Tropykus, ordinals, etc: (proyectos RSK o crypto que se quieran auspiciar entre bitcoiners.).

Key featuers:
  - Estamos participando de la hackaton de RSK.
  - Queremos hacer que usuarios de nostr te hagan publicidad.
  - Vamos a estar pagandoles a ellos en DoC (cosa que con LN no se puede).
  - Te vamos a dar un reporte de campaña con todos los mensajes que se enviaron.
  - Vamos a gastar 200 DoC. Los ponemos nosotros por anticipado. solo nos pagás si te gusta el resultado.
    * Tenemos 1000 DoC de presupuesto total.

Outreach Influencers:
  - Buscamos cuentas notables.

## Tareas
- Nubis + Rodrigo: Crear kanban y CRM, con algunos protocols y persona de contacto.
- Gera + Rodrigo: equipo comercial, contactar a los prospects del CRM con nuestra propuesta, acomodado a cada caso.
- Limbert: Scouting de hashtags, campañas, posibles prospects del mundo Nostr.
- Benja + Limbert: Búsqueda o armado de repositorio de influencers / cuentas notables de nostr para contactar. (otro CRM).
- Nubis: Preparar sesión hands-on crear un robot de nostr.

