
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.model.FailureHandling.OPTIONAL
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;
import static org.mockito.Mockito.verify

import org.eclipse.persistence.internal.jpa.config.mappings.AbstractBasicMappingImpl

import com.kms.katalon.core.annotation.Keyword;
import com.kms.katalon.core.checkpoint.Checkpoint;
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords;
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords;
import com.kms.katalon.core.model.FailureHandling;
import com.kms.katalon.core.testcase.TestCase;
import com.kms.katalon.core.testdata.TestData;
import com.kms.katalon.core.testobject.TestObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords;
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

public class Portal {
	//Mail mail = new Mail()
	String hesloMail = GlobalVariable.mailHeslo

	/**
	 * Prihlási sa na portál ako rodič alebo osoba ktorá koná za SaSZ
	 *
	 * @param ucet                 Číslo účtu.
	 * @param heslo            		Heslo k účtu.
	 * @param F2A                 	2-faktorová autentifikácia. Default je vypnutá.
	 * @param riaditel             	Typ účtu. Default je osobný.
	 */

	def prihlasUcet(String ucet, String heslo, boolean F2A = false, boolean riaditel = false) {
		WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Prihlsi sa'))

		WebUI.click(findTestObject('Object Repository/Zak_test/Page_- Iam.Web/h3_Identifiktor a heslo fyzickej osoby'))

		WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input__DomainName'),
				ucet)

		WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input__Password'),
				heslo)

		WebUI.click(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input_Zabudli ste heslo_btn-continue'))

		if (F2A) {
			//delay pred zacatim
			//WebUI.delay(20)
			//Zistenie overovaciseho kódu z mailu'
			def kod = mail.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', 'katalontest987@gmail.com', hesloMail)
			WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__OneTimeCode'), kod.toString())
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_Overovac kd - Iam.Web/input__btn-continue'))
		}

		if (riaditel) {
			//pokracuje ako riaditel
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_ePrihlky/button_Pokraova'))
		} else if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_ePrihlky/input_(nepovinn)_JeZamestnanec'), 5, FailureHandling.OPTIONAL)){
			//oznaci sa moznost "osobny"
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_ePrihlky/input_(nepovinn)_JeZamestnanec'))
			WebUI.click(findTestObject('Object Repository/Zak_test/Page_ePrihlky/button_Pokraova'))
		}
	}

	def prihlasUcetBez2FA(String ucet, String heslo) {
		WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Prihlsi sa'))

		WebUI.click(findTestObject('Object Repository/Zak_test/Page_- Iam.Web/h3_Identifiktor a heslo fyzickej osoby'))

		WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input__DomainName'),
				ucet)

		WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input__Password'),
				heslo)

		WebUI.click(findTestObject('Object Repository/Zak_test/Page_Identifiktor a heslo fyzickej osoby - Iam.Web/input_Zabudli ste heslo_btn-continue'))
	}

	def nacitajNahodnyZaznam(String nazovSuboru) {
		def riadky = []
		try {
			new File(nazovSuboru).eachLine { riadok ->
				riadky << riadok
			}
		} catch (IOException e) {
			println "Chyba pri čítaní súboru: ${e.getMessage()}"
			return null
		}

		if (riadky) {
			def nahodnyIndex = new RandomData().nextInt(riadky.size())
			def nahodnyRiadok = riadky[nahodnyIndex]
			def udaje = nahodnyRiadok.split(';')
			if (udaje.size() == 4) {
				return [meno: udaje[0], priezvisko: udaje[1], cislo: udaje[2], pohlavie: udaje[3]]
			} else {
				println "Nesprávny formát riadku v súbore: ${nahodnyRiadok}"
				return null
			}
		} else {
			println "Súbor je prázdny."
			return null
		}
	}

	/*def vyberUdaje(String nazovSuboru) {
	 def riadky = []
	 try {
	 new File(nazovSuboru).eachLine { riadok ->
	 riadky << riadok
	 }
	 } catch (IOException e) {
	 println "Chyba pri čítaní súboru: ${e.getMessage()}"
	 return null
	 }
	 if (riadky) {
	 def nahodnyIndex = new Random().nextInt(riadky.size())
	 def nahodnyRiadok = riadky[nahodnyIndex]
	 def udaje = nahodnyRiadok.split(';')
	 if (udaje.size() == 3) {
	 return [meno: udaje[0], priezvisko: udaje[1], pohlavie: udaje[2]]
	 } else {
	 println "Nesprávny formát riadku v súbore: ${nahodnyRiadok}"
	 return null
	 }
	 } else {
	 println "Súbor je prázdny."
	 return null
	 }
	 }
	 */
	def vyberUdaje(String nazovSuboru) {
		def riadky = []
		File subor = new File(nazovSuboru)
		try {

			if (!subor.exists()) {
				println "Súbor neexistuje: ${nazovSuboru}"
				return null
			}
			riadky = subor.readLines()
		} catch (IOException e) {
			println "Chyba pri čítaní súboru: ${e.getMessage()}"
			return null
		}

		if (riadky) {
			def nahodnyIndex = new RandomData().nextInt(riadky.size())
			def nahodnyRiadok = riadky.remove(nahodnyIndex) // Remove the random line from the list

			try {
				// Explicitly write lines back to the file using a standard approach
				// Join the lines with a newline character and write as a single string
				subor.write(riadky.join('\n'))
				// If you need to ensure the last line also has a newline, or if your original lines already
				// contain them and you want to preserve exact original line endings,
				// you might want to adjust join accordingly.
				// For most text files, '\n' is sufficient.
			} catch (IOException e) {
				println "Chyba pri zápise do súboru po odstránení riadku: ${e.getMessage()}"
				return null
			}

			def udaje = nahodnyRiadok.split(';')
			if (udaje.size() == 3) {
				return [meno: udaje[0], priezvisko: udaje[1], pohlavie: udaje[2]]
			} else {
				println "Nesprávny formát vybratého riadku v súbore (riadok bol napriek tomu odstránený): ${nahodnyRiadok}"
				return null
			}
		} else {
			println "Súbor je prázdny alebo po odstránení riadku ostal prázdny."
			return null
		}
	}
}