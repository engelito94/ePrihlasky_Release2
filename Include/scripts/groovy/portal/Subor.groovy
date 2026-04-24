package portal;
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.model.FailureHandling.OPTIONAL
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;
import static org.mockito.Mockito.verify

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

public class Subor {
	/**
	 * Nacíta a vymaze náhodný záznam zo súboru
	 *
	 * @param nazovSuboru           Názov .txt.
	 * 
	 * @return	Vráti dáta (meno, priezvisko, RC, cislo RC)	
	 */
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
			def nahodnyIndex = new Random().nextInt(riadky.size())
			def nahodnyRiadok = riadky[nahodnyIndex]
			// Odstranenie použitého riadku zo zoznamu
			riadky.remove(nahodnyIndex)
			// Prepísanie súboru bez použitého riadku
			new File(nazovSuboru).withWriter { writer ->
				riadky.each { line ->
					writer.writeLine(line)
				}
			}
			def udaje = nahodnyRiadok.split(';')
			if (udaje.size() == 4) {
				return [meno: udaje[0], priezvisko: udaje[1], rc: udaje[2], cislo: udaje[3]]
			} else {
				println "Nesprávny formát riadku v súbore: ${nahodnyRiadok}"
				return null
			}
		} else {
			println "Súbor je prázdny."
			return null
		}
	}

	def dajDietaRiadSS(String nazovSuboru) {
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
			// Odstranenie použitého riadku zo zoznamu
			riadky.remove(nahodnyIndex)
			// Prepísanie súboru bez použitého riadku
			new File(nazovSuboru).withWriter { writer ->
				riadky.each { line ->
					writer.writeLine(line)
				}
			}
			def udaje = nahodnyRiadok.split(';')
			if (udaje.size() >= 3) {
				return [meno: udaje[0], priezvisko: udaje[1], rc: udaje[2]]
			} else {
				println "Nesprávny formát riadku v súbore: ${nahodnyRiadok}"
				return null
			}
		} else {
			println "Súbor je prázdny."
			return null
		}
	}

	def zapisUdajeNaPrenos(String meno, String priezvisko) {
		File f = new File("Data Files/prenosUdajov.txt")
		f.write("${meno}|${priezvisko}")
	}

	def precitajPreneseneUdaje() {
		File f = new File("Data Files/prenosUdajov.txt")
		if (f.exists()) {
			String riadok = f.getText()
			def parts = riadok.split("\\|")
			String meno = parts[0]
			String priezvisko = parts[1]

			return [meno: parts[0], priezvisko: parts[1]]
		} else {
			throw new RuntimeException("Súbor prenosUdajov.txt neexistuje!")
		}
	}
}