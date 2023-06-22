<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_CloudTablesLow code DataTables and Edi_841c9b</name>
   <tag></tag>
   <elementGuidId>5f9a23d3-9934-4988-85d6-16aca9ba3e4c</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body.wide.comments.example</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>920a71c3-04b8-46a8-8fdb-1d47c56cdb1f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>wide comments example</value>
      <webElementGuid>22a00f3e-b3ba-49de-958a-fd2898985e26</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
	
		
	
	
		
			
				
					
						CloudTables
						
							Low code DataTables and Editor. Configured in your browser in moments.
						
					
					
						DataTables
						
							Advanced interaction
							features for your tables.
						
					
					
						Editor
						
							Comprehensive editing
							library for DataTables.
						
					
				
				
					
						Manual
					
					
						Download
					
					Login / Register
					
						
							×
						
					
				
			
			
				Monetize your audience: Fund an OSS project or website with EthicalAds, a privacy-first ad networkAds by EthicalAds
			
		
		
			
				ExamplesBasic initialisationAdvanced initialisationStylingData sourcesAPIAjaxServer-sidePlug-insManualReferenceExtensionsPlug-insBlogForumsSupportFAQsDownloadPurchase
			
			
				≡ Show site navigation
			
		
		
			
				Form inputs
				
					In order to perform paging, ordering, searching etc, DataTables can remove rows and cells from the document (i.e. those rows / cells which are not needed
					are not inserted into the document). This increases performance and compatibility, however, it means that submitting forms which span multiple pages requires a
					little bit of additional work to get the information that is not in the document any longer.
					The $() method can be used to get nodes from the
					document regardless of paging, ordering etc. This example shows $() being used to get all input elements from the table. In the example a simple
					alert() is used to show the information from the form, but an Ajax call to the server with the form data could easily be performed.
					If you are interested in a complete CRUD editing suit for DataTables have a look at the Editor extension which
					provides simple setup and complete integration with DataTables.
				
				
					Submit form
					Show 102550100 entriesSearch:
						
							NameAgePositionOffice
						
						
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
						
								Airi Satou
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Angelica Ramos
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Ashton Cox
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bradley Greer
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brenden Wagner
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brielle Williamson
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bruno Nash
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Caesar Vance
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cara Stevens
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cedric Kelly
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
						
							NameAgePositionOffice
						
					Showing 1 to 10 of 57 entriesPrevious123456Next
				
				
					Javascript
					HTML
					CSS
					Ajax
					Server-side script
					Comments (0)
				
				
					
						The Javascript shown below is used to initialise the table shown in this example:Javascript12345678910111213141516$(document).ready(function () {    var table = $('#example').DataTable({        columnDefs: [            {                orderable: false,                targets: [1, 2, 3],            },        ],    });     $('button').click(function () {        var data = table.$('input, select').serialize();        alert('The following data would have been submitted to the server: \n\n' + data.substr(0, 120) + '...');        return false;    });});
						In addition to the above code, the following Javascript library files are loaded for use in this example:
						
							
								https://code.jquery.com/jquery-3.5.1.js
							
							
								https://cdn.datatables.net/1.13.4/js/jquery.dataTables.min.js
							
						
					
					
						The HTML shown below is the raw HTML table element, before it has been enhanced by DataTables:
					HTML123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100101102103104105106107108109110111112113114115116117118119120121122123124125126127128129130131132133134135136137138139140141142143144145146147148149150151152153154155156157158159160161162163164165166167168169170171172173174175176177178179180181182183184185186187188189190191192193194195196197198199200201202203204205206207208209210211212213214215216217218219220221222223224225226227228229230231232233234235236237238239240241242243244245246247248249250251252253254255256257258259260261262263264265266267268269270271272273274275276277278279280281282283284285286287288289290291292293294295296297298299300301302303304305306307308309310311312313314315316317318319320321322323324325326327328329330331332333334335336337338339340341342343344345346347348349350351352353354355356357358359360361362363364365366367368369370371372373374375376377378379380381382383384385386387388389390391392393394395396397398399400401402403404405406407408409410411412413414415416417418419420421422423424425426427428429430431432433434435436437438439440441442443444445446447448449450451452453454455456457458459460461462463464465466467468469470471472473474475476477478479480481482483484485486487488489490491492493494495496497498499500501502503504505506507508509510511512513514515516517518519520521522523524525526527528529530531532533534535536537538539540541542543544545546547548549550551552553554555556557558559560561562563564565566567568569570571572573574575576577578579580581582583584585586587588589590591592593594595596597598599600601602603604605606607608609610611612613614615616617618619620621622623624625626627628629630631632633634635636637638639640641642643644645646647648649650651652653654655656657658659660661662663664665666667668669670671672673674675676677678679680681682683684685686687688689690691692693694695696697698699700701702703704705706707708709710711712713714715716717718719720721722723724725726727728729730731732733734735736737738739740741742743744745746747748749750751752753754755756757758759760761762763764765766767768769770771772773774775776777778779780781782783784785786787788789790791792793794795796797798799800801802803804805806807808809810811812813814815816817818819820821822823824825826827828829830831832833834835836837838839840841842843844845846847848849850851852853854855856857858859860861862863864865866867868869870871872873874875876877878879880881882883884885886887888889890891892893894895896897898899900901902903904905906907908909910911912913914915916917918919920921922923924925926927928929930931932933934935936937938939940941942943944945946947948949950951952953954955956957958959960961962963964965966967968969970971972973974975976977978979980981982983984985986987988989990991992993994995996997998999100010011002100310041005100610071008100910101011101210131014101510161017101810191020102110221023102410251026102710281029103010311032103310341035103610371038103910401041104210431044104510461047104810491050105110521053105410551056105710581059106010611062106310641065106610671068106910701071107210731074107510761077107810791080108110821083108410851086108710881089109010911092109310941095109610971098109911001101110211031104110511061107110811091110111111121113111411151116111711181119112011211122112311241125112611271128112911301131113211331134113511361137113811391140114111421143114411451146114711481149115011511152115311541155115611571158115911601161116211631164116511661167116811691170117111721173117411751176117711781179118011811182118311841185118611871188118911901191119211931194119511961197119811991200120112021203120412051206120712081209121012111212121312141215121612171218121912201221122212231224122512261227122812291230123112321233123412351236123712381239124012411242124312441245124612471248124912501251125212531254125512561257125812591260126112621263126412651266126712681269127012711272127312741275&lt;button type=&quot;submit&quot;>Submit form&lt;/button>    &lt;table id=&quot;example&quot; class=&quot;display&quot; style=&quot;width:100%&quot;>        &lt;thead>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/thead>        &lt;tbody>            &lt;tr>                &lt;td>Tiger Nixon&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-age&quot; name=&quot;row-1-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-position&quot; name=&quot;row-1-position&quot; value=&quot;System Architect&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-1-office&quot; name=&quot;row-1-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Garrett Winters&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-age&quot; name=&quot;row-2-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-position&quot; name=&quot;row-2-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-2-office&quot; name=&quot;row-2-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Ashton Cox&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-age&quot; name=&quot;row-3-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-position&quot; name=&quot;row-3-position&quot; value=&quot;Junior Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-3-office&quot; name=&quot;row-3-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cedric Kelly&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-age&quot; name=&quot;row-4-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-position&quot; name=&quot;row-4-position&quot; value=&quot;Senior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-4-office&quot; name=&quot;row-4-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Airi Satou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-age&quot; name=&quot;row-5-age&quot; value=&quot;33&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-position&quot; name=&quot;row-5-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-5-office&quot; name=&quot;row-5-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brielle Williamson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-age&quot; name=&quot;row-6-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-position&quot; name=&quot;row-6-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-6-office&quot; name=&quot;row-6-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Herrod Chandler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-age&quot; name=&quot;row-7-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-position&quot; name=&quot;row-7-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-7-office&quot; name=&quot;row-7-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Rhona Davidson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-age&quot; name=&quot;row-8-age&quot; value=&quot;55&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-position&quot; name=&quot;row-8-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-8-office&quot; name=&quot;row-8-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Colleen Hurst&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-age&quot; name=&quot;row-9-age&quot; value=&quot;39&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-position&quot; name=&quot;row-9-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-9-office&quot; name=&quot;row-9-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sonya Frost&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-age&quot; name=&quot;row-10-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-position&quot; name=&quot;row-10-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-10-office&quot; name=&quot;row-10-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jena Gaines&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-age&quot; name=&quot;row-11-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-position&quot; name=&quot;row-11-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-11-office&quot; name=&quot;row-11-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Quinn Flynn&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-age&quot; name=&quot;row-12-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-position&quot; name=&quot;row-12-position&quot; value=&quot;Support Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-12-office&quot; name=&quot;row-12-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Charde Marshall&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-age&quot; name=&quot;row-13-age&quot; value=&quot;36&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-position&quot; name=&quot;row-13-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-13-office&quot; name=&quot;row-13-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Haley Kennedy&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-age&quot; name=&quot;row-14-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-position&quot; name=&quot;row-14-position&quot; value=&quot;Senior Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-14-office&quot; name=&quot;row-14-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Tatyana Fitzpatrick&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-age&quot; name=&quot;row-15-age&quot; value=&quot;19&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-position&quot; name=&quot;row-15-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-15-office&quot; name=&quot;row-15-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Silva&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-age&quot; name=&quot;row-16-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-position&quot; name=&quot;row-16-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-16-office&quot; name=&quot;row-16-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Paul Byrd&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-age&quot; name=&quot;row-17-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-position&quot; name=&quot;row-17-position&quot; value=&quot;Chief Financial Officer (CFO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-17-office&quot; name=&quot;row-17-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gloria Little&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-age&quot; name=&quot;row-18-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-position&quot; name=&quot;row-18-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-18-office&quot; name=&quot;row-18-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bradley Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-age&quot; name=&quot;row-19-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-position&quot; name=&quot;row-19-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-19-office&quot; name=&quot;row-19-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Dai Rios&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-age&quot; name=&quot;row-20-age&quot; value=&quot;35&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-position&quot; name=&quot;row-20-position&quot; value=&quot;Personnel Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-20-office&quot; name=&quot;row-20-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jenette Caldwell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-age&quot; name=&quot;row-21-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-position&quot; name=&quot;row-21-position&quot; value=&quot;Development Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-21-office&quot; name=&quot;row-21-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Yuri Berry&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-age&quot; name=&quot;row-22-age&quot; value=&quot;40&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-position&quot; name=&quot;row-22-position&quot; value=&quot;Chief Marketing Officer (CMO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-22-office&quot; name=&quot;row-22-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Caesar Vance&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-age&quot; name=&quot;row-23-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-position&quot; name=&quot;row-23-position&quot; value=&quot;Pre-Sales Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-23-office&quot; name=&quot;row-23-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Doris Wilder&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-age&quot; name=&quot;row-24-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-position&quot; name=&quot;row-24-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-24-office&quot; name=&quot;row-24-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Angelica Ramos&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-age&quot; name=&quot;row-25-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-position&quot; name=&quot;row-25-position&quot; value=&quot;Chief Executive Officer (CEO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-25-office&quot; name=&quot;row-25-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Joyce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-age&quot; name=&quot;row-26-age&quot; value=&quot;42&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-position&quot; name=&quot;row-26-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-26-office&quot; name=&quot;row-26-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Chang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-age&quot; name=&quot;row-27-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-position&quot; name=&quot;row-27-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-27-office&quot; name=&quot;row-27-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brenden Wagner&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-age&quot; name=&quot;row-28-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-position&quot; name=&quot;row-28-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-28-office&quot; name=&quot;row-28-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Fiona Green&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-age&quot; name=&quot;row-29-age&quot; value=&quot;48&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-position&quot; name=&quot;row-29-position&quot; value=&quot;Chief Operating Officer (COO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-29-office&quot; name=&quot;row-29-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shou Itou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-age&quot; name=&quot;row-30-age&quot; value=&quot;20&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-position&quot; name=&quot;row-30-position&quot; value=&quot;Regional Marketing&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-30-office&quot; name=&quot;row-30-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michelle House&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-age&quot; name=&quot;row-31-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-position&quot; name=&quot;row-31-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-31-office&quot; name=&quot;row-31-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Suki Burks&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-age&quot; name=&quot;row-32-age&quot; value=&quot;53&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-position&quot; name=&quot;row-32-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-32-office&quot; name=&quot;row-32-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Prescott Bartlett&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-age&quot; name=&quot;row-33-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-position&quot; name=&quot;row-33-position&quot; value=&quot;Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-33-office&quot; name=&quot;row-33-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Cortez&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-age&quot; name=&quot;row-34-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-position&quot; name=&quot;row-34-position&quot; value=&quot;Team Leader&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-34-office&quot; name=&quot;row-34-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Martena Mccray&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-age&quot; name=&quot;row-35-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-position&quot; name=&quot;row-35-position&quot; value=&quot;Post-Sales support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-35-office&quot; name=&quot;row-35-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Unity Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-age&quot; name=&quot;row-36-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-position&quot; name=&quot;row-36-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-36-office&quot; name=&quot;row-36-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Howard Hatfield&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-age&quot; name=&quot;row-37-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-position&quot; name=&quot;row-37-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-37-office&quot; name=&quot;row-37-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hope Fuentes&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-age&quot; name=&quot;row-38-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-position&quot; name=&quot;row-38-position&quot; value=&quot;Secretary&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-38-office&quot; name=&quot;row-38-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Vivian Harrell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-age&quot; name=&quot;row-39-age&quot; value=&quot;62&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-position&quot; name=&quot;row-39-position&quot; value=&quot;Financial Controller&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-39-office&quot; name=&quot;row-39-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Timothy Mooney&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-age&quot; name=&quot;row-40-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-position&quot; name=&quot;row-40-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-40-office&quot; name=&quot;row-40-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jackson Bradshaw&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-age&quot; name=&quot;row-41-age&quot; value=&quot;65&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-position&quot; name=&quot;row-41-position&quot; value=&quot;Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-41-office&quot; name=&quot;row-41-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Olivia Liang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-age&quot; name=&quot;row-42-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-position&quot; name=&quot;row-42-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-42-office&quot; name=&quot;row-42-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bruno Nash&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-age&quot; name=&quot;row-43-age&quot; value=&quot;38&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-position&quot; name=&quot;row-43-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-43-office&quot; name=&quot;row-43-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sakura Yamamoto&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-age&quot; name=&quot;row-44-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-position&quot; name=&quot;row-44-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-44-office&quot; name=&quot;row-44-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Thor Walton&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-age&quot; name=&quot;row-45-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-position&quot; name=&quot;row-45-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-45-office&quot; name=&quot;row-45-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Finn Camacho&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-age&quot; name=&quot;row-46-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-position&quot; name=&quot;row-46-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-46-office&quot; name=&quot;row-46-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Serge Baldwin&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-age&quot; name=&quot;row-47-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-position&quot; name=&quot;row-47-position&quot; value=&quot;Data Coordinator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-47-office&quot; name=&quot;row-47-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zenaida Frank&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-age&quot; name=&quot;row-48-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-position&quot; name=&quot;row-48-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-48-office&quot; name=&quot;row-48-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zorita Serrano&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-age&quot; name=&quot;row-49-age&quot; value=&quot;56&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-position&quot; name=&quot;row-49-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-49-office&quot; name=&quot;row-49-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Acosta&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-age&quot; name=&quot;row-50-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-position&quot; name=&quot;row-50-position&quot; value=&quot;Junior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-50-office&quot; name=&quot;row-50-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cara Stevens&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-age&quot; name=&quot;row-51-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-position&quot; name=&quot;row-51-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-51-office&quot; name=&quot;row-51-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hermione Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-age&quot; name=&quot;row-52-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-position&quot; name=&quot;row-52-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-52-office&quot; name=&quot;row-52-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Lael Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-age&quot; name=&quot;row-53-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-position&quot; name=&quot;row-53-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-53-office&quot; name=&quot;row-53-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jonas Alexander&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-age&quot; name=&quot;row-54-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-position&quot; name=&quot;row-54-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-54-office&quot; name=&quot;row-54-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shad Decker&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-age&quot; name=&quot;row-55-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-position&quot; name=&quot;row-55-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-55-office&quot; name=&quot;row-55-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Bruce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-age&quot; name=&quot;row-56-age&quot; value=&quot;29&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-position&quot; name=&quot;row-56-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-56-office&quot; name=&quot;row-56-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Donna Snider&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-age&quot; name=&quot;row-57-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-position&quot; name=&quot;row-57-position&quot; value=&quot;Customer Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-57-office&quot; name=&quot;row-57-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>        &lt;/tbody>        &lt;tfoot>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/tfoot>    &lt;/table>
					
						
							This example uses a little bit of additional CSS beyond what is loaded from the library files (below), in order to correctly display the table. The
							additional CSS used is shown below:CSS1 
						
						The following CSS library files are loaded for use in this example to provide the styling of the table:
						
							
								https://cdn.datatables.net/1.13.4/css/jquery.dataTables.min.css
							
						
					
					
						This table loads data by Ajax. The latest data that has been loaded is shown below. This data will update automatically as any additional data is
						loaded.
					
					
						The script used to perform the server-side processing for this table is shown below. Please note that this is just an example script using PHP.
						Server-side processing scripts can be written in any language, using the protocol described in the DataTables
						documentation.
					
					No comments posted for this page yet. Be the first to contribute!Post new commentContributions in the form of tips, code snippets and suggestions for the above material are very welcome. To post a comment, please use the form below. Text is formatted by Markdown.To post comments, please sign in to your DataTables account, or register:Sign inRegisterAny questions posted here will be deleted without being published.Please post questions in the Forums. Comments are moderated.
				
				Other examples
				
					
						Basic initialisation
						
							
								Zero configuration
							
							
								Feature enable / disable
							
							
								Default ordering (sorting)
							
							
								Multi-column ordering
							
							
								Multiple tables
							
							
								Hidden columns
							
							
								Complex headers (rowspan and colspan)
							
							
								DOM positioning
							
							
								Flexible table width
							
							
								State saving
							
							
								Alternative pagination
							
							
								Data rendering
							
							
								Scroll - vertical
							
							
								Scroll - vertical, dynamic height
							
							
								Scroll - horizontal
							
							
								Scroll - horizontal and vertical
							
							
								Language - Comma decimal place
							
						
					
					
						Non-jQuery initialisation
						
							
								Non-jQuery initialisation
							
							
								Options
							
							
								DOM events
							
							
								DataTables events
							
							
								Ajax data source (objects)
							
						
					
					
						Advanced initialisation
						
							
								DOM / jQuery events
							
							
								DataTables events
							
							
								Column rendering
							
							
								Enter Key to Search
							
							
								Page length options
							
							
								Multiple table control elements
							
							
								Complex headers with column visibility
							
							
								Read HTML to data objects
							
							
								HTML5 data-* attributes - cell data
							
							
								HTML5 data-* attributes - table options
							
							
								Setting defaults
							
							
								Row created callback
							
							
								Row grouping
							
							
								Footer callback
							
							
								Custom toolbar elements
							
							
								Order direction sequence control
							
							
								Example of stocks results
							
						
					
					
						Data sources
						
							
								HTML (DOM) sourced data
							
							
								Ajax sourced data
							
							
								Javascript sourced data
							
							
								Server-side processing
							
						
					
					
						Internationalisation
						
							
								Language options
							
							
								Remote language file
							
							
								Remote language file + local definitions
							
							
								Locale based number display
							
							
								Auto-locale display
							
						
					
					
						DateTime
						
							
								ISO8601 detection
							
							
								Auto-locale display (Moment.js)
							
							
								Auto-locale display (Luxon)
							
							
								Date rendering (Moment.js)
							
							
								Date rendering (Luxon)
							
							
								Format transform (Moment.js)
							
							
								Format transform (Luxon)
							
							
								Ordering formatted dates (Moment.js)
							
							
								Ordering formatted dates (Luxon)
							
						
					
					
						Styling
						
							
								Base style
							
							
								Base style - no styling classes
							
							
								Base style - cell borders
							
							
								Base style - compact
							
							
								Base style - hover
							
							
								Base style - order-column
							
							
								Base style - row borders
							
							
								Base style - stripe
							
							
								Bootstrap 3
							
							
								Bootstrap 4
							
							
								Bootstrap 5
							
							
								Foundation
							
							
								Fomantic-UI (formally Semantic-UI)
							
							
								Bulma
							
							
								jQuery UI ThemeRoller
							
							
								Material Design (Tech. preview)
							
							
								UIKit 3 (Tech. preview)
							
						
					
					
						API
						
							
								Add rows
							
							
								Individual column searching (text inputs)
							
							
								Individual column searching (select inputs)
							
							
								Highlighting rows and columns
							
							
								Child rows (show extra / detailed information)
							
							
								Child rows with StateSave
							
							
								Row selection (multiple rows)
							
							
								Row selection and deletion (single row)
							
							
								Form inputs
							
							
								Index column
							
							
								Show / hide columns dynamically
							
							
								Using API in callbacks
							
							
								Scrolling and Bootstrap tabs
							
							
								Search API (regular expressions)
							
							
								HighCharts Integration
							
						
					
					
						Ajax
						
							
								Ajax data source (arrays)
							
							
								Ajax data source (objects)
							
							
								Nested object data (objects)
							
							
								Nested object data (arrays)
							
							
								Orthogonal data
							
							
								Generated content for a column
							
							
								Custom data source property
							
							
								Flat array data source
							
							
								Deferred rendering for speed
							
						
					
					
						Server-side
						
							
								Server-side processing
							
							
								Custom HTTP variables
							
							
								POST data
							
							
								Return key to search
							
							
								Automatic addition of row ID attributes
							
							
								Object data source
							
							
								Row details
							
							
								JSONP data source for remote domains
							
							
								Deferred loading of data
							
							
								Pipelining data to reduce Ajax calls for paging
							
						
					
					
						Plug-ins
						
							
								API plug-in methods
							
							
								Ordering plug-ins (with type detection)
							
							
								Ordering plug-ins (no type detection)
							
							
								Custom filtering - range search
							
							
								Live DOM ordering
							
						
					
				
			
		
	
	
		
			DataTables
			DataTables designed and created by SpryMedia Ltd.
			© 2007-2023 MIT licensed. Privacy policy. Supporters.
			SpryMedia Ltd is registered in Scotland, company no. SC456502.
		
	

/html[1]/body[@class=&quot;wide comments example&quot;]</value>
      <webElementGuid>c3a7c00b-7a7a-45a2-a4ef-767afb2b4dad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;wide comments example&quot;]</value>
      <webElementGuid>ef34f42b-8bd0-4219-adf7-8bbf833c5816</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>ca5c2220-6b0a-431f-8c79-4a989385c458</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
	
	
		
	
	
		
			
				
					
						CloudTables
						
							Low code DataTables and Editor. Configured in your browser in moments.
						
					
					
						DataTables
						
							Advanced interaction
							features for your tables.
						
					
					
						Editor
						
							Comprehensive editing
							library for DataTables.
						
					
				
				
					
						Manual
					
					
						Download
					
					Login / Register
					
						
							×
						
					
				
			
			
				Monetize your audience: Fund an OSS project or website with EthicalAds, a privacy-first ad networkAds by EthicalAds
			
		
		
			
				ExamplesBasic initialisationAdvanced initialisationStylingData sourcesAPIAjaxServer-sidePlug-insManualReferenceExtensionsPlug-insBlogForumsSupportFAQsDownloadPurchase
			
			
				≡ Show site navigation
			
		
		
			
				Form inputs
				
					In order to perform paging, ordering, searching etc, DataTables can remove rows and cells from the document (i.e. those rows / cells which are not needed
					are not inserted into the document). This increases performance and compatibility, however, it means that submitting forms which span multiple pages requires a
					little bit of additional work to get the information that is not in the document any longer.
					The $() method can be used to get nodes from the
					document regardless of paging, ordering etc. This example shows $() being used to get all input elements from the table. In the example a simple
					alert() is used to show the information from the form, but an Ajax call to the server with the form data could easily be performed.
					If you are interested in a complete CRUD editing suit for DataTables have a look at the Editor extension which
					provides simple setup and complete integration with DataTables.
				
				
					Submit form
					Show 102550100 entriesSearch:
						
							NameAgePositionOffice
						
						
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
						
								Airi Satou
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Angelica Ramos
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Ashton Cox
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bradley Greer
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brenden Wagner
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brielle Williamson
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bruno Nash
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Caesar Vance
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cara Stevens
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cedric Kelly
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
						
							NameAgePositionOffice
						
					Showing 1 to 10 of 57 entriesPrevious123456Next
				
				
					Javascript
					HTML
					CSS
					Ajax
					Server-side script
					Comments (0)
				
				
					
						The Javascript shown below is used to initialise the table shown in this example:Javascript12345678910111213141516$(document).ready(function () {    var table = $(&quot; , &quot;'&quot; , &quot;#example&quot; , &quot;'&quot; , &quot;).DataTable({        columnDefs: [            {                orderable: false,                targets: [1, 2, 3],            },        ],    });     $(&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;).click(function () {        var data = table.$(&quot; , &quot;'&quot; , &quot;input, select&quot; , &quot;'&quot; , &quot;).serialize();        alert(&quot; , &quot;'&quot; , &quot;The following data would have been submitted to the server: \n\n&quot; , &quot;'&quot; , &quot; + data.substr(0, 120) + &quot; , &quot;'&quot; , &quot;...&quot; , &quot;'&quot; , &quot;);        return false;    });});
						In addition to the above code, the following Javascript library files are loaded for use in this example:
						
							
								https://code.jquery.com/jquery-3.5.1.js
							
							
								https://cdn.datatables.net/1.13.4/js/jquery.dataTables.min.js
							
						
					
					
						The HTML shown below is the raw HTML table element, before it has been enhanced by DataTables:
					HTML123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100101102103104105106107108109110111112113114115116117118119120121122123124125126127128129130131132133134135136137138139140141142143144145146147148149150151152153154155156157158159160161162163164165166167168169170171172173174175176177178179180181182183184185186187188189190191192193194195196197198199200201202203204205206207208209210211212213214215216217218219220221222223224225226227228229230231232233234235236237238239240241242243244245246247248249250251252253254255256257258259260261262263264265266267268269270271272273274275276277278279280281282283284285286287288289290291292293294295296297298299300301302303304305306307308309310311312313314315316317318319320321322323324325326327328329330331332333334335336337338339340341342343344345346347348349350351352353354355356357358359360361362363364365366367368369370371372373374375376377378379380381382383384385386387388389390391392393394395396397398399400401402403404405406407408409410411412413414415416417418419420421422423424425426427428429430431432433434435436437438439440441442443444445446447448449450451452453454455456457458459460461462463464465466467468469470471472473474475476477478479480481482483484485486487488489490491492493494495496497498499500501502503504505506507508509510511512513514515516517518519520521522523524525526527528529530531532533534535536537538539540541542543544545546547548549550551552553554555556557558559560561562563564565566567568569570571572573574575576577578579580581582583584585586587588589590591592593594595596597598599600601602603604605606607608609610611612613614615616617618619620621622623624625626627628629630631632633634635636637638639640641642643644645646647648649650651652653654655656657658659660661662663664665666667668669670671672673674675676677678679680681682683684685686687688689690691692693694695696697698699700701702703704705706707708709710711712713714715716717718719720721722723724725726727728729730731732733734735736737738739740741742743744745746747748749750751752753754755756757758759760761762763764765766767768769770771772773774775776777778779780781782783784785786787788789790791792793794795796797798799800801802803804805806807808809810811812813814815816817818819820821822823824825826827828829830831832833834835836837838839840841842843844845846847848849850851852853854855856857858859860861862863864865866867868869870871872873874875876877878879880881882883884885886887888889890891892893894895896897898899900901902903904905906907908909910911912913914915916917918919920921922923924925926927928929930931932933934935936937938939940941942943944945946947948949950951952953954955956957958959960961962963964965966967968969970971972973974975976977978979980981982983984985986987988989990991992993994995996997998999100010011002100310041005100610071008100910101011101210131014101510161017101810191020102110221023102410251026102710281029103010311032103310341035103610371038103910401041104210431044104510461047104810491050105110521053105410551056105710581059106010611062106310641065106610671068106910701071107210731074107510761077107810791080108110821083108410851086108710881089109010911092109310941095109610971098109911001101110211031104110511061107110811091110111111121113111411151116111711181119112011211122112311241125112611271128112911301131113211331134113511361137113811391140114111421143114411451146114711481149115011511152115311541155115611571158115911601161116211631164116511661167116811691170117111721173117411751176117711781179118011811182118311841185118611871188118911901191119211931194119511961197119811991200120112021203120412051206120712081209121012111212121312141215121612171218121912201221122212231224122512261227122812291230123112321233123412351236123712381239124012411242124312441245124612471248124912501251125212531254125512561257125812591260126112621263126412651266126712681269127012711272127312741275&lt;button type=&quot;submit&quot;>Submit form&lt;/button>    &lt;table id=&quot;example&quot; class=&quot;display&quot; style=&quot;width:100%&quot;>        &lt;thead>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/thead>        &lt;tbody>            &lt;tr>                &lt;td>Tiger Nixon&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-age&quot; name=&quot;row-1-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-position&quot; name=&quot;row-1-position&quot; value=&quot;System Architect&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-1-office&quot; name=&quot;row-1-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Garrett Winters&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-age&quot; name=&quot;row-2-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-position&quot; name=&quot;row-2-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-2-office&quot; name=&quot;row-2-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Ashton Cox&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-age&quot; name=&quot;row-3-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-position&quot; name=&quot;row-3-position&quot; value=&quot;Junior Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-3-office&quot; name=&quot;row-3-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cedric Kelly&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-age&quot; name=&quot;row-4-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-position&quot; name=&quot;row-4-position&quot; value=&quot;Senior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-4-office&quot; name=&quot;row-4-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Airi Satou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-age&quot; name=&quot;row-5-age&quot; value=&quot;33&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-position&quot; name=&quot;row-5-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-5-office&quot; name=&quot;row-5-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brielle Williamson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-age&quot; name=&quot;row-6-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-position&quot; name=&quot;row-6-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-6-office&quot; name=&quot;row-6-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Herrod Chandler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-age&quot; name=&quot;row-7-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-position&quot; name=&quot;row-7-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-7-office&quot; name=&quot;row-7-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Rhona Davidson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-age&quot; name=&quot;row-8-age&quot; value=&quot;55&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-position&quot; name=&quot;row-8-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-8-office&quot; name=&quot;row-8-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Colleen Hurst&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-age&quot; name=&quot;row-9-age&quot; value=&quot;39&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-position&quot; name=&quot;row-9-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-9-office&quot; name=&quot;row-9-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sonya Frost&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-age&quot; name=&quot;row-10-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-position&quot; name=&quot;row-10-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-10-office&quot; name=&quot;row-10-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jena Gaines&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-age&quot; name=&quot;row-11-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-position&quot; name=&quot;row-11-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-11-office&quot; name=&quot;row-11-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Quinn Flynn&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-age&quot; name=&quot;row-12-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-position&quot; name=&quot;row-12-position&quot; value=&quot;Support Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-12-office&quot; name=&quot;row-12-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Charde Marshall&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-age&quot; name=&quot;row-13-age&quot; value=&quot;36&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-position&quot; name=&quot;row-13-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-13-office&quot; name=&quot;row-13-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Haley Kennedy&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-age&quot; name=&quot;row-14-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-position&quot; name=&quot;row-14-position&quot; value=&quot;Senior Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-14-office&quot; name=&quot;row-14-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Tatyana Fitzpatrick&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-age&quot; name=&quot;row-15-age&quot; value=&quot;19&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-position&quot; name=&quot;row-15-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-15-office&quot; name=&quot;row-15-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Silva&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-age&quot; name=&quot;row-16-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-position&quot; name=&quot;row-16-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-16-office&quot; name=&quot;row-16-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Paul Byrd&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-age&quot; name=&quot;row-17-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-position&quot; name=&quot;row-17-position&quot; value=&quot;Chief Financial Officer (CFO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-17-office&quot; name=&quot;row-17-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gloria Little&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-age&quot; name=&quot;row-18-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-position&quot; name=&quot;row-18-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-18-office&quot; name=&quot;row-18-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bradley Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-age&quot; name=&quot;row-19-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-position&quot; name=&quot;row-19-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-19-office&quot; name=&quot;row-19-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Dai Rios&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-age&quot; name=&quot;row-20-age&quot; value=&quot;35&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-position&quot; name=&quot;row-20-position&quot; value=&quot;Personnel Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-20-office&quot; name=&quot;row-20-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jenette Caldwell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-age&quot; name=&quot;row-21-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-position&quot; name=&quot;row-21-position&quot; value=&quot;Development Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-21-office&quot; name=&quot;row-21-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Yuri Berry&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-age&quot; name=&quot;row-22-age&quot; value=&quot;40&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-position&quot; name=&quot;row-22-position&quot; value=&quot;Chief Marketing Officer (CMO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-22-office&quot; name=&quot;row-22-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Caesar Vance&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-age&quot; name=&quot;row-23-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-position&quot; name=&quot;row-23-position&quot; value=&quot;Pre-Sales Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-23-office&quot; name=&quot;row-23-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Doris Wilder&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-age&quot; name=&quot;row-24-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-position&quot; name=&quot;row-24-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-24-office&quot; name=&quot;row-24-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Angelica Ramos&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-age&quot; name=&quot;row-25-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-position&quot; name=&quot;row-25-position&quot; value=&quot;Chief Executive Officer (CEO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-25-office&quot; name=&quot;row-25-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Joyce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-age&quot; name=&quot;row-26-age&quot; value=&quot;42&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-position&quot; name=&quot;row-26-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-26-office&quot; name=&quot;row-26-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Chang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-age&quot; name=&quot;row-27-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-position&quot; name=&quot;row-27-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-27-office&quot; name=&quot;row-27-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brenden Wagner&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-age&quot; name=&quot;row-28-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-position&quot; name=&quot;row-28-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-28-office&quot; name=&quot;row-28-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Fiona Green&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-age&quot; name=&quot;row-29-age&quot; value=&quot;48&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-position&quot; name=&quot;row-29-position&quot; value=&quot;Chief Operating Officer (COO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-29-office&quot; name=&quot;row-29-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shou Itou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-age&quot; name=&quot;row-30-age&quot; value=&quot;20&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-position&quot; name=&quot;row-30-position&quot; value=&quot;Regional Marketing&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-30-office&quot; name=&quot;row-30-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michelle House&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-age&quot; name=&quot;row-31-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-position&quot; name=&quot;row-31-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-31-office&quot; name=&quot;row-31-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Suki Burks&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-age&quot; name=&quot;row-32-age&quot; value=&quot;53&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-position&quot; name=&quot;row-32-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-32-office&quot; name=&quot;row-32-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Prescott Bartlett&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-age&quot; name=&quot;row-33-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-position&quot; name=&quot;row-33-position&quot; value=&quot;Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-33-office&quot; name=&quot;row-33-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Cortez&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-age&quot; name=&quot;row-34-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-position&quot; name=&quot;row-34-position&quot; value=&quot;Team Leader&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-34-office&quot; name=&quot;row-34-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Martena Mccray&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-age&quot; name=&quot;row-35-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-position&quot; name=&quot;row-35-position&quot; value=&quot;Post-Sales support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-35-office&quot; name=&quot;row-35-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Unity Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-age&quot; name=&quot;row-36-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-position&quot; name=&quot;row-36-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-36-office&quot; name=&quot;row-36-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Howard Hatfield&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-age&quot; name=&quot;row-37-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-position&quot; name=&quot;row-37-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-37-office&quot; name=&quot;row-37-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hope Fuentes&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-age&quot; name=&quot;row-38-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-position&quot; name=&quot;row-38-position&quot; value=&quot;Secretary&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-38-office&quot; name=&quot;row-38-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Vivian Harrell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-age&quot; name=&quot;row-39-age&quot; value=&quot;62&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-position&quot; name=&quot;row-39-position&quot; value=&quot;Financial Controller&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-39-office&quot; name=&quot;row-39-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Timothy Mooney&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-age&quot; name=&quot;row-40-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-position&quot; name=&quot;row-40-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-40-office&quot; name=&quot;row-40-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jackson Bradshaw&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-age&quot; name=&quot;row-41-age&quot; value=&quot;65&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-position&quot; name=&quot;row-41-position&quot; value=&quot;Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-41-office&quot; name=&quot;row-41-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Olivia Liang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-age&quot; name=&quot;row-42-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-position&quot; name=&quot;row-42-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-42-office&quot; name=&quot;row-42-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bruno Nash&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-age&quot; name=&quot;row-43-age&quot; value=&quot;38&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-position&quot; name=&quot;row-43-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-43-office&quot; name=&quot;row-43-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sakura Yamamoto&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-age&quot; name=&quot;row-44-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-position&quot; name=&quot;row-44-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-44-office&quot; name=&quot;row-44-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Thor Walton&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-age&quot; name=&quot;row-45-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-position&quot; name=&quot;row-45-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-45-office&quot; name=&quot;row-45-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Finn Camacho&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-age&quot; name=&quot;row-46-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-position&quot; name=&quot;row-46-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-46-office&quot; name=&quot;row-46-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Serge Baldwin&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-age&quot; name=&quot;row-47-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-position&quot; name=&quot;row-47-position&quot; value=&quot;Data Coordinator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-47-office&quot; name=&quot;row-47-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zenaida Frank&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-age&quot; name=&quot;row-48-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-position&quot; name=&quot;row-48-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-48-office&quot; name=&quot;row-48-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zorita Serrano&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-age&quot; name=&quot;row-49-age&quot; value=&quot;56&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-position&quot; name=&quot;row-49-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-49-office&quot; name=&quot;row-49-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Acosta&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-age&quot; name=&quot;row-50-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-position&quot; name=&quot;row-50-position&quot; value=&quot;Junior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-50-office&quot; name=&quot;row-50-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cara Stevens&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-age&quot; name=&quot;row-51-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-position&quot; name=&quot;row-51-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-51-office&quot; name=&quot;row-51-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hermione Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-age&quot; name=&quot;row-52-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-position&quot; name=&quot;row-52-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-52-office&quot; name=&quot;row-52-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Lael Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-age&quot; name=&quot;row-53-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-position&quot; name=&quot;row-53-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-53-office&quot; name=&quot;row-53-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jonas Alexander&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-age&quot; name=&quot;row-54-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-position&quot; name=&quot;row-54-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-54-office&quot; name=&quot;row-54-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shad Decker&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-age&quot; name=&quot;row-55-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-position&quot; name=&quot;row-55-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-55-office&quot; name=&quot;row-55-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Bruce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-age&quot; name=&quot;row-56-age&quot; value=&quot;29&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-position&quot; name=&quot;row-56-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-56-office&quot; name=&quot;row-56-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Donna Snider&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-age&quot; name=&quot;row-57-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-position&quot; name=&quot;row-57-position&quot; value=&quot;Customer Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-57-office&quot; name=&quot;row-57-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>        &lt;/tbody>        &lt;tfoot>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/tfoot>    &lt;/table>
					
						
							This example uses a little bit of additional CSS beyond what is loaded from the library files (below), in order to correctly display the table. The
							additional CSS used is shown below:CSS1 
						
						The following CSS library files are loaded for use in this example to provide the styling of the table:
						
							
								https://cdn.datatables.net/1.13.4/css/jquery.dataTables.min.css
							
						
					
					
						This table loads data by Ajax. The latest data that has been loaded is shown below. This data will update automatically as any additional data is
						loaded.
					
					
						The script used to perform the server-side processing for this table is shown below. Please note that this is just an example script using PHP.
						Server-side processing scripts can be written in any language, using the protocol described in the DataTables
						documentation.
					
					No comments posted for this page yet. Be the first to contribute!Post new commentContributions in the form of tips, code snippets and suggestions for the above material are very welcome. To post a comment, please use the form below. Text is formatted by Markdown.To post comments, please sign in to your DataTables account, or register:Sign inRegisterAny questions posted here will be deleted without being published.Please post questions in the Forums. Comments are moderated.
				
				Other examples
				
					
						Basic initialisation
						
							
								Zero configuration
							
							
								Feature enable / disable
							
							
								Default ordering (sorting)
							
							
								Multi-column ordering
							
							
								Multiple tables
							
							
								Hidden columns
							
							
								Complex headers (rowspan and colspan)
							
							
								DOM positioning
							
							
								Flexible table width
							
							
								State saving
							
							
								Alternative pagination
							
							
								Data rendering
							
							
								Scroll - vertical
							
							
								Scroll - vertical, dynamic height
							
							
								Scroll - horizontal
							
							
								Scroll - horizontal and vertical
							
							
								Language - Comma decimal place
							
						
					
					
						Non-jQuery initialisation
						
							
								Non-jQuery initialisation
							
							
								Options
							
							
								DOM events
							
							
								DataTables events
							
							
								Ajax data source (objects)
							
						
					
					
						Advanced initialisation
						
							
								DOM / jQuery events
							
							
								DataTables events
							
							
								Column rendering
							
							
								Enter Key to Search
							
							
								Page length options
							
							
								Multiple table control elements
							
							
								Complex headers with column visibility
							
							
								Read HTML to data objects
							
							
								HTML5 data-* attributes - cell data
							
							
								HTML5 data-* attributes - table options
							
							
								Setting defaults
							
							
								Row created callback
							
							
								Row grouping
							
							
								Footer callback
							
							
								Custom toolbar elements
							
							
								Order direction sequence control
							
							
								Example of stocks results
							
						
					
					
						Data sources
						
							
								HTML (DOM) sourced data
							
							
								Ajax sourced data
							
							
								Javascript sourced data
							
							
								Server-side processing
							
						
					
					
						Internationalisation
						
							
								Language options
							
							
								Remote language file
							
							
								Remote language file + local definitions
							
							
								Locale based number display
							
							
								Auto-locale display
							
						
					
					
						DateTime
						
							
								ISO8601 detection
							
							
								Auto-locale display (Moment.js)
							
							
								Auto-locale display (Luxon)
							
							
								Date rendering (Moment.js)
							
							
								Date rendering (Luxon)
							
							
								Format transform (Moment.js)
							
							
								Format transform (Luxon)
							
							
								Ordering formatted dates (Moment.js)
							
							
								Ordering formatted dates (Luxon)
							
						
					
					
						Styling
						
							
								Base style
							
							
								Base style - no styling classes
							
							
								Base style - cell borders
							
							
								Base style - compact
							
							
								Base style - hover
							
							
								Base style - order-column
							
							
								Base style - row borders
							
							
								Base style - stripe
							
							
								Bootstrap 3
							
							
								Bootstrap 4
							
							
								Bootstrap 5
							
							
								Foundation
							
							
								Fomantic-UI (formally Semantic-UI)
							
							
								Bulma
							
							
								jQuery UI ThemeRoller
							
							
								Material Design (Tech. preview)
							
							
								UIKit 3 (Tech. preview)
							
						
					
					
						API
						
							
								Add rows
							
							
								Individual column searching (text inputs)
							
							
								Individual column searching (select inputs)
							
							
								Highlighting rows and columns
							
							
								Child rows (show extra / detailed information)
							
							
								Child rows with StateSave
							
							
								Row selection (multiple rows)
							
							
								Row selection and deletion (single row)
							
							
								Form inputs
							
							
								Index column
							
							
								Show / hide columns dynamically
							
							
								Using API in callbacks
							
							
								Scrolling and Bootstrap tabs
							
							
								Search API (regular expressions)
							
							
								HighCharts Integration
							
						
					
					
						Ajax
						
							
								Ajax data source (arrays)
							
							
								Ajax data source (objects)
							
							
								Nested object data (objects)
							
							
								Nested object data (arrays)
							
							
								Orthogonal data
							
							
								Generated content for a column
							
							
								Custom data source property
							
							
								Flat array data source
							
							
								Deferred rendering for speed
							
						
					
					
						Server-side
						
							
								Server-side processing
							
							
								Custom HTTP variables
							
							
								POST data
							
							
								Return key to search
							
							
								Automatic addition of row ID attributes
							
							
								Object data source
							
							
								Row details
							
							
								JSONP data source for remote domains
							
							
								Deferred loading of data
							
							
								Pipelining data to reduce Ajax calls for paging
							
						
					
					
						Plug-ins
						
							
								API plug-in methods
							
							
								Ordering plug-ins (with type detection)
							
							
								Ordering plug-ins (no type detection)
							
							
								Custom filtering - range search
							
							
								Live DOM ordering
							
						
					
				
			
		
	
	
		
			DataTables
			DataTables designed and created by SpryMedia Ltd.
			© 2007-2023 MIT licensed. Privacy policy. Supporters.
			SpryMedia Ltd is registered in Scotland, company no. SC456502.
		
	

/html[1]/body[@class=&quot;wide comments example&quot;]&quot;) or . = concat(&quot;
	
	
		
	
	
		
			
				
					
						CloudTables
						
							Low code DataTables and Editor. Configured in your browser in moments.
						
					
					
						DataTables
						
							Advanced interaction
							features for your tables.
						
					
					
						Editor
						
							Comprehensive editing
							library for DataTables.
						
					
				
				
					
						Manual
					
					
						Download
					
					Login / Register
					
						
							×
						
					
				
			
			
				Monetize your audience: Fund an OSS project or website with EthicalAds, a privacy-first ad networkAds by EthicalAds
			
		
		
			
				ExamplesBasic initialisationAdvanced initialisationStylingData sourcesAPIAjaxServer-sidePlug-insManualReferenceExtensionsPlug-insBlogForumsSupportFAQsDownloadPurchase
			
			
				≡ Show site navigation
			
		
		
			
				Form inputs
				
					In order to perform paging, ordering, searching etc, DataTables can remove rows and cells from the document (i.e. those rows / cells which are not needed
					are not inserted into the document). This increases performance and compatibility, however, it means that submitting forms which span multiple pages requires a
					little bit of additional work to get the information that is not in the document any longer.
					The $() method can be used to get nodes from the
					document regardless of paging, ordering etc. This example shows $() being used to get all input elements from the table. In the example a simple
					alert() is used to show the information from the form, but an Ajax call to the server with the form data could easily be performed.
					If you are interested in a complete CRUD editing suit for DataTables have a look at the Editor extension which
					provides simple setup and complete integration with DataTables.
				
				
					Submit form
					Show 102550100 entriesSearch:
						
							NameAgePositionOffice
						
						
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
							
						
								Airi Satou
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Angelica Ramos
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Ashton Cox
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bradley Greer
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brenden Wagner
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Brielle Williamson
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Bruno Nash
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Caesar Vance
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cara Stevens
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
								Cedric Kelly
								
								
								
									
										Edinburgh
									
									
										London
									
									
										New York
									
									
										San Francisco
									
									
										Tokyo
									
								
							
						
							NameAgePositionOffice
						
					Showing 1 to 10 of 57 entriesPrevious123456Next
				
				
					Javascript
					HTML
					CSS
					Ajax
					Server-side script
					Comments (0)
				
				
					
						The Javascript shown below is used to initialise the table shown in this example:Javascript12345678910111213141516$(document).ready(function () {    var table = $(&quot; , &quot;'&quot; , &quot;#example&quot; , &quot;'&quot; , &quot;).DataTable({        columnDefs: [            {                orderable: false,                targets: [1, 2, 3],            },        ],    });     $(&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;).click(function () {        var data = table.$(&quot; , &quot;'&quot; , &quot;input, select&quot; , &quot;'&quot; , &quot;).serialize();        alert(&quot; , &quot;'&quot; , &quot;The following data would have been submitted to the server: \n\n&quot; , &quot;'&quot; , &quot; + data.substr(0, 120) + &quot; , &quot;'&quot; , &quot;...&quot; , &quot;'&quot; , &quot;);        return false;    });});
						In addition to the above code, the following Javascript library files are loaded for use in this example:
						
							
								https://code.jquery.com/jquery-3.5.1.js
							
							
								https://cdn.datatables.net/1.13.4/js/jquery.dataTables.min.js
							
						
					
					
						The HTML shown below is the raw HTML table element, before it has been enhanced by DataTables:
					HTML123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100101102103104105106107108109110111112113114115116117118119120121122123124125126127128129130131132133134135136137138139140141142143144145146147148149150151152153154155156157158159160161162163164165166167168169170171172173174175176177178179180181182183184185186187188189190191192193194195196197198199200201202203204205206207208209210211212213214215216217218219220221222223224225226227228229230231232233234235236237238239240241242243244245246247248249250251252253254255256257258259260261262263264265266267268269270271272273274275276277278279280281282283284285286287288289290291292293294295296297298299300301302303304305306307308309310311312313314315316317318319320321322323324325326327328329330331332333334335336337338339340341342343344345346347348349350351352353354355356357358359360361362363364365366367368369370371372373374375376377378379380381382383384385386387388389390391392393394395396397398399400401402403404405406407408409410411412413414415416417418419420421422423424425426427428429430431432433434435436437438439440441442443444445446447448449450451452453454455456457458459460461462463464465466467468469470471472473474475476477478479480481482483484485486487488489490491492493494495496497498499500501502503504505506507508509510511512513514515516517518519520521522523524525526527528529530531532533534535536537538539540541542543544545546547548549550551552553554555556557558559560561562563564565566567568569570571572573574575576577578579580581582583584585586587588589590591592593594595596597598599600601602603604605606607608609610611612613614615616617618619620621622623624625626627628629630631632633634635636637638639640641642643644645646647648649650651652653654655656657658659660661662663664665666667668669670671672673674675676677678679680681682683684685686687688689690691692693694695696697698699700701702703704705706707708709710711712713714715716717718719720721722723724725726727728729730731732733734735736737738739740741742743744745746747748749750751752753754755756757758759760761762763764765766767768769770771772773774775776777778779780781782783784785786787788789790791792793794795796797798799800801802803804805806807808809810811812813814815816817818819820821822823824825826827828829830831832833834835836837838839840841842843844845846847848849850851852853854855856857858859860861862863864865866867868869870871872873874875876877878879880881882883884885886887888889890891892893894895896897898899900901902903904905906907908909910911912913914915916917918919920921922923924925926927928929930931932933934935936937938939940941942943944945946947948949950951952953954955956957958959960961962963964965966967968969970971972973974975976977978979980981982983984985986987988989990991992993994995996997998999100010011002100310041005100610071008100910101011101210131014101510161017101810191020102110221023102410251026102710281029103010311032103310341035103610371038103910401041104210431044104510461047104810491050105110521053105410551056105710581059106010611062106310641065106610671068106910701071107210731074107510761077107810791080108110821083108410851086108710881089109010911092109310941095109610971098109911001101110211031104110511061107110811091110111111121113111411151116111711181119112011211122112311241125112611271128112911301131113211331134113511361137113811391140114111421143114411451146114711481149115011511152115311541155115611571158115911601161116211631164116511661167116811691170117111721173117411751176117711781179118011811182118311841185118611871188118911901191119211931194119511961197119811991200120112021203120412051206120712081209121012111212121312141215121612171218121912201221122212231224122512261227122812291230123112321233123412351236123712381239124012411242124312441245124612471248124912501251125212531254125512561257125812591260126112621263126412651266126712681269127012711272127312741275&lt;button type=&quot;submit&quot;>Submit form&lt;/button>    &lt;table id=&quot;example&quot; class=&quot;display&quot; style=&quot;width:100%&quot;>        &lt;thead>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/thead>        &lt;tbody>            &lt;tr>                &lt;td>Tiger Nixon&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-age&quot; name=&quot;row-1-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-1-position&quot; name=&quot;row-1-position&quot; value=&quot;System Architect&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-1-office&quot; name=&quot;row-1-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Garrett Winters&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-age&quot; name=&quot;row-2-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-2-position&quot; name=&quot;row-2-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-2-office&quot; name=&quot;row-2-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Ashton Cox&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-age&quot; name=&quot;row-3-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-3-position&quot; name=&quot;row-3-position&quot; value=&quot;Junior Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-3-office&quot; name=&quot;row-3-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cedric Kelly&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-age&quot; name=&quot;row-4-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-4-position&quot; name=&quot;row-4-position&quot; value=&quot;Senior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-4-office&quot; name=&quot;row-4-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Airi Satou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-age&quot; name=&quot;row-5-age&quot; value=&quot;33&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-5-position&quot; name=&quot;row-5-position&quot; value=&quot;Accountant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-5-office&quot; name=&quot;row-5-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brielle Williamson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-age&quot; name=&quot;row-6-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-6-position&quot; name=&quot;row-6-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-6-office&quot; name=&quot;row-6-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Herrod Chandler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-age&quot; name=&quot;row-7-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-7-position&quot; name=&quot;row-7-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-7-office&quot; name=&quot;row-7-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Rhona Davidson&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-age&quot; name=&quot;row-8-age&quot; value=&quot;55&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-8-position&quot; name=&quot;row-8-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-8-office&quot; name=&quot;row-8-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Colleen Hurst&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-age&quot; name=&quot;row-9-age&quot; value=&quot;39&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-9-position&quot; name=&quot;row-9-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-9-office&quot; name=&quot;row-9-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sonya Frost&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-age&quot; name=&quot;row-10-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-10-position&quot; name=&quot;row-10-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-10-office&quot; name=&quot;row-10-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jena Gaines&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-age&quot; name=&quot;row-11-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-11-position&quot; name=&quot;row-11-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-11-office&quot; name=&quot;row-11-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Quinn Flynn&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-age&quot; name=&quot;row-12-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-12-position&quot; name=&quot;row-12-position&quot; value=&quot;Support Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-12-office&quot; name=&quot;row-12-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Charde Marshall&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-age&quot; name=&quot;row-13-age&quot; value=&quot;36&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-13-position&quot; name=&quot;row-13-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-13-office&quot; name=&quot;row-13-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Haley Kennedy&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-age&quot; name=&quot;row-14-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-14-position&quot; name=&quot;row-14-position&quot; value=&quot;Senior Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-14-office&quot; name=&quot;row-14-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Tatyana Fitzpatrick&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-age&quot; name=&quot;row-15-age&quot; value=&quot;19&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-15-position&quot; name=&quot;row-15-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-15-office&quot; name=&quot;row-15-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Silva&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-age&quot; name=&quot;row-16-age&quot; value=&quot;66&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-16-position&quot; name=&quot;row-16-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-16-office&quot; name=&quot;row-16-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Paul Byrd&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-age&quot; name=&quot;row-17-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-17-position&quot; name=&quot;row-17-position&quot; value=&quot;Chief Financial Officer (CFO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-17-office&quot; name=&quot;row-17-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gloria Little&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-age&quot; name=&quot;row-18-age&quot; value=&quot;59&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-18-position&quot; name=&quot;row-18-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-18-office&quot; name=&quot;row-18-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bradley Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-age&quot; name=&quot;row-19-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-19-position&quot; name=&quot;row-19-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-19-office&quot; name=&quot;row-19-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Dai Rios&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-age&quot; name=&quot;row-20-age&quot; value=&quot;35&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-20-position&quot; name=&quot;row-20-position&quot; value=&quot;Personnel Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-20-office&quot; name=&quot;row-20-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jenette Caldwell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-age&quot; name=&quot;row-21-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-21-position&quot; name=&quot;row-21-position&quot; value=&quot;Development Lead&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-21-office&quot; name=&quot;row-21-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Yuri Berry&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-age&quot; name=&quot;row-22-age&quot; value=&quot;40&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-22-position&quot; name=&quot;row-22-position&quot; value=&quot;Chief Marketing Officer (CMO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-22-office&quot; name=&quot;row-22-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Caesar Vance&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-age&quot; name=&quot;row-23-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-23-position&quot; name=&quot;row-23-position&quot; value=&quot;Pre-Sales Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-23-office&quot; name=&quot;row-23-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Doris Wilder&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-age&quot; name=&quot;row-24-age&quot; value=&quot;23&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-24-position&quot; name=&quot;row-24-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-24-office&quot; name=&quot;row-24-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Angelica Ramos&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-age&quot; name=&quot;row-25-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-25-position&quot; name=&quot;row-25-position&quot; value=&quot;Chief Executive Officer (CEO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-25-office&quot; name=&quot;row-25-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Joyce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-age&quot; name=&quot;row-26-age&quot; value=&quot;42&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-26-position&quot; name=&quot;row-26-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-26-office&quot; name=&quot;row-26-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Chang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-age&quot; name=&quot;row-27-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-27-position&quot; name=&quot;row-27-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-27-office&quot; name=&quot;row-27-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Brenden Wagner&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-age&quot; name=&quot;row-28-age&quot; value=&quot;28&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-28-position&quot; name=&quot;row-28-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-28-office&quot; name=&quot;row-28-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Fiona Green&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-age&quot; name=&quot;row-29-age&quot; value=&quot;48&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-29-position&quot; name=&quot;row-29-position&quot; value=&quot;Chief Operating Officer (COO)&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-29-office&quot; name=&quot;row-29-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shou Itou&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-age&quot; name=&quot;row-30-age&quot; value=&quot;20&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-30-position&quot; name=&quot;row-30-position&quot; value=&quot;Regional Marketing&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-30-office&quot; name=&quot;row-30-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michelle House&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-age&quot; name=&quot;row-31-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-31-position&quot; name=&quot;row-31-position&quot; value=&quot;Integration Specialist&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-31-office&quot; name=&quot;row-31-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Suki Burks&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-age&quot; name=&quot;row-32-age&quot; value=&quot;53&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-32-position&quot; name=&quot;row-32-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-32-office&quot; name=&quot;row-32-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Prescott Bartlett&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-age&quot; name=&quot;row-33-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-33-position&quot; name=&quot;row-33-position&quot; value=&quot;Technical Author&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-33-office&quot; name=&quot;row-33-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Gavin Cortez&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-age&quot; name=&quot;row-34-age&quot; value=&quot;22&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-34-position&quot; name=&quot;row-34-position&quot; value=&quot;Team Leader&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-34-office&quot; name=&quot;row-34-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Martena Mccray&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-age&quot; name=&quot;row-35-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-35-position&quot; name=&quot;row-35-position&quot; value=&quot;Post-Sales support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-35-office&quot; name=&quot;row-35-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Unity Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-age&quot; name=&quot;row-36-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-36-position&quot; name=&quot;row-36-position&quot; value=&quot;Marketing Designer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-36-office&quot; name=&quot;row-36-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Howard Hatfield&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-age&quot; name=&quot;row-37-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-37-position&quot; name=&quot;row-37-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-37-office&quot; name=&quot;row-37-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hope Fuentes&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-age&quot; name=&quot;row-38-age&quot; value=&quot;41&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-38-position&quot; name=&quot;row-38-position&quot; value=&quot;Secretary&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-38-office&quot; name=&quot;row-38-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Vivian Harrell&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-age&quot; name=&quot;row-39-age&quot; value=&quot;62&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-39-position&quot; name=&quot;row-39-position&quot; value=&quot;Financial Controller&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-39-office&quot; name=&quot;row-39-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Timothy Mooney&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-age&quot; name=&quot;row-40-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-40-position&quot; name=&quot;row-40-position&quot; value=&quot;Office Manager&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-40-office&quot; name=&quot;row-40-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jackson Bradshaw&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-age&quot; name=&quot;row-41-age&quot; value=&quot;65&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-41-position&quot; name=&quot;row-41-position&quot; value=&quot;Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-41-office&quot; name=&quot;row-41-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Olivia Liang&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-age&quot; name=&quot;row-42-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-42-position&quot; name=&quot;row-42-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-42-office&quot; name=&quot;row-42-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Bruno Nash&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-age&quot; name=&quot;row-43-age&quot; value=&quot;38&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-43-position&quot; name=&quot;row-43-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-43-office&quot; name=&quot;row-43-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Sakura Yamamoto&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-age&quot; name=&quot;row-44-age&quot; value=&quot;37&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-44-position&quot; name=&quot;row-44-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-44-office&quot; name=&quot;row-44-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot; selected=&quot;selected&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Thor Walton&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-age&quot; name=&quot;row-45-age&quot; value=&quot;61&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-45-position&quot; name=&quot;row-45-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-45-office&quot; name=&quot;row-45-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Finn Camacho&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-age&quot; name=&quot;row-46-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-46-position&quot; name=&quot;row-46-position&quot; value=&quot;Support Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-46-office&quot; name=&quot;row-46-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Serge Baldwin&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-age&quot; name=&quot;row-47-age&quot; value=&quot;64&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-47-position&quot; name=&quot;row-47-position&quot; value=&quot;Data Coordinator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-47-office&quot; name=&quot;row-47-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zenaida Frank&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-age&quot; name=&quot;row-48-age&quot; value=&quot;63&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-48-position&quot; name=&quot;row-48-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-48-office&quot; name=&quot;row-48-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Zorita Serrano&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-age&quot; name=&quot;row-49-age&quot; value=&quot;56&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-49-position&quot; name=&quot;row-49-position&quot; value=&quot;Software Engineer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-49-office&quot; name=&quot;row-49-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jennifer Acosta&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-age&quot; name=&quot;row-50-age&quot; value=&quot;43&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-50-position&quot; name=&quot;row-50-position&quot; value=&quot;Junior Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-50-office&quot; name=&quot;row-50-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Cara Stevens&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-age&quot; name=&quot;row-51-age&quot; value=&quot;46&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-51-position&quot; name=&quot;row-51-position&quot; value=&quot;Sales Assistant&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-51-office&quot; name=&quot;row-51-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Hermione Butler&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-age&quot; name=&quot;row-52-age&quot; value=&quot;47&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-52-position&quot; name=&quot;row-52-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-52-office&quot; name=&quot;row-52-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Lael Greer&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-age&quot; name=&quot;row-53-age&quot; value=&quot;21&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-53-position&quot; name=&quot;row-53-position&quot; value=&quot;Systems Administrator&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-53-office&quot; name=&quot;row-53-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot; selected=&quot;selected&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Jonas Alexander&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-age&quot; name=&quot;row-54-age&quot; value=&quot;30&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-54-position&quot; name=&quot;row-54-position&quot; value=&quot;Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-54-office&quot; name=&quot;row-54-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot; selected=&quot;selected&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Shad Decker&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-age&quot; name=&quot;row-55-age&quot; value=&quot;51&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-55-position&quot; name=&quot;row-55-position&quot; value=&quot;Regional Director&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-55-office&quot; name=&quot;row-55-office&quot;>                    &lt;option value=&quot;Edinburgh&quot; selected=&quot;selected&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Michael Bruce&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-age&quot; name=&quot;row-56-age&quot; value=&quot;29&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-56-position&quot; name=&quot;row-56-position&quot; value=&quot;Javascript Developer&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-56-office&quot; name=&quot;row-56-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>            &lt;tr>                &lt;td>Donna Snider&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-age&quot; name=&quot;row-57-age&quot; value=&quot;27&quot;>&lt;/td>                &lt;td>&lt;input type=&quot;text&quot; id=&quot;row-57-position&quot; name=&quot;row-57-position&quot; value=&quot;Customer Support&quot;>&lt;/td>                &lt;td>&lt;select size=&quot;1&quot; id=&quot;row-57-office&quot; name=&quot;row-57-office&quot;>                    &lt;option value=&quot;Edinburgh&quot;>                        Edinburgh                    &lt;/option>                    &lt;option value=&quot;London&quot;>                        London                    &lt;/option>                    &lt;option value=&quot;New York&quot; selected=&quot;selected&quot;>                        New York                    &lt;/option>                    &lt;option value=&quot;San Francisco&quot;>                        San Francisco                    &lt;/option>                    &lt;option value=&quot;Tokyo&quot;>                        Tokyo                    &lt;/option>                &lt;/select>&lt;/td>            &lt;/tr>        &lt;/tbody>        &lt;tfoot>            &lt;tr>                &lt;th>Name&lt;/th>                &lt;th>Age&lt;/th>                &lt;th>Position&lt;/th>                &lt;th>Office&lt;/th>            &lt;/tr>        &lt;/tfoot>    &lt;/table>
					
						
							This example uses a little bit of additional CSS beyond what is loaded from the library files (below), in order to correctly display the table. The
							additional CSS used is shown below:CSS1 
						
						The following CSS library files are loaded for use in this example to provide the styling of the table:
						
							
								https://cdn.datatables.net/1.13.4/css/jquery.dataTables.min.css
							
						
					
					
						This table loads data by Ajax. The latest data that has been loaded is shown below. This data will update automatically as any additional data is
						loaded.
					
					
						The script used to perform the server-side processing for this table is shown below. Please note that this is just an example script using PHP.
						Server-side processing scripts can be written in any language, using the protocol described in the DataTables
						documentation.
					
					No comments posted for this page yet. Be the first to contribute!Post new commentContributions in the form of tips, code snippets and suggestions for the above material are very welcome. To post a comment, please use the form below. Text is formatted by Markdown.To post comments, please sign in to your DataTables account, or register:Sign inRegisterAny questions posted here will be deleted without being published.Please post questions in the Forums. Comments are moderated.
				
				Other examples
				
					
						Basic initialisation
						
							
								Zero configuration
							
							
								Feature enable / disable
							
							
								Default ordering (sorting)
							
							
								Multi-column ordering
							
							
								Multiple tables
							
							
								Hidden columns
							
							
								Complex headers (rowspan and colspan)
							
							
								DOM positioning
							
							
								Flexible table width
							
							
								State saving
							
							
								Alternative pagination
							
							
								Data rendering
							
							
								Scroll - vertical
							
							
								Scroll - vertical, dynamic height
							
							
								Scroll - horizontal
							
							
								Scroll - horizontal and vertical
							
							
								Language - Comma decimal place
							
						
					
					
						Non-jQuery initialisation
						
							
								Non-jQuery initialisation
							
							
								Options
							
							
								DOM events
							
							
								DataTables events
							
							
								Ajax data source (objects)
							
						
					
					
						Advanced initialisation
						
							
								DOM / jQuery events
							
							
								DataTables events
							
							
								Column rendering
							
							
								Enter Key to Search
							
							
								Page length options
							
							
								Multiple table control elements
							
							
								Complex headers with column visibility
							
							
								Read HTML to data objects
							
							
								HTML5 data-* attributes - cell data
							
							
								HTML5 data-* attributes - table options
							
							
								Setting defaults
							
							
								Row created callback
							
							
								Row grouping
							
							
								Footer callback
							
							
								Custom toolbar elements
							
							
								Order direction sequence control
							
							
								Example of stocks results
							
						
					
					
						Data sources
						
							
								HTML (DOM) sourced data
							
							
								Ajax sourced data
							
							
								Javascript sourced data
							
							
								Server-side processing
							
						
					
					
						Internationalisation
						
							
								Language options
							
							
								Remote language file
							
							
								Remote language file + local definitions
							
							
								Locale based number display
							
							
								Auto-locale display
							
						
					
					
						DateTime
						
							
								ISO8601 detection
							
							
								Auto-locale display (Moment.js)
							
							
								Auto-locale display (Luxon)
							
							
								Date rendering (Moment.js)
							
							
								Date rendering (Luxon)
							
							
								Format transform (Moment.js)
							
							
								Format transform (Luxon)
							
							
								Ordering formatted dates (Moment.js)
							
							
								Ordering formatted dates (Luxon)
							
						
					
					
						Styling
						
							
								Base style
							
							
								Base style - no styling classes
							
							
								Base style - cell borders
							
							
								Base style - compact
							
							
								Base style - hover
							
							
								Base style - order-column
							
							
								Base style - row borders
							
							
								Base style - stripe
							
							
								Bootstrap 3
							
							
								Bootstrap 4
							
							
								Bootstrap 5
							
							
								Foundation
							
							
								Fomantic-UI (formally Semantic-UI)
							
							
								Bulma
							
							
								jQuery UI ThemeRoller
							
							
								Material Design (Tech. preview)
							
							
								UIKit 3 (Tech. preview)
							
						
					
					
						API
						
							
								Add rows
							
							
								Individual column searching (text inputs)
							
							
								Individual column searching (select inputs)
							
							
								Highlighting rows and columns
							
							
								Child rows (show extra / detailed information)
							
							
								Child rows with StateSave
							
							
								Row selection (multiple rows)
							
							
								Row selection and deletion (single row)
							
							
								Form inputs
							
							
								Index column
							
							
								Show / hide columns dynamically
							
							
								Using API in callbacks
							
							
								Scrolling and Bootstrap tabs
							
							
								Search API (regular expressions)
							
							
								HighCharts Integration
							
						
					
					
						Ajax
						
							
								Ajax data source (arrays)
							
							
								Ajax data source (objects)
							
							
								Nested object data (objects)
							
							
								Nested object data (arrays)
							
							
								Orthogonal data
							
							
								Generated content for a column
							
							
								Custom data source property
							
							
								Flat array data source
							
							
								Deferred rendering for speed
							
						
					
					
						Server-side
						
							
								Server-side processing
							
							
								Custom HTTP variables
							
							
								POST data
							
							
								Return key to search
							
							
								Automatic addition of row ID attributes
							
							
								Object data source
							
							
								Row details
							
							
								JSONP data source for remote domains
							
							
								Deferred loading of data
							
							
								Pipelining data to reduce Ajax calls for paging
							
						
					
					
						Plug-ins
						
							
								API plug-in methods
							
							
								Ordering plug-ins (with type detection)
							
							
								Ordering plug-ins (no type detection)
							
							
								Custom filtering - range search
							
							
								Live DOM ordering
							
						
					
				
			
		
	
	
		
			DataTables
			DataTables designed and created by SpryMedia Ltd.
			© 2007-2023 MIT licensed. Privacy policy. Supporters.
			SpryMedia Ltd is registered in Scotland, company no. SC456502.
		
	

/html[1]/body[@class=&quot;wide comments example&quot;]&quot;))]</value>
      <webElementGuid>0d6b64c5-a9c5-4fd2-906e-9f8fb9210822</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
